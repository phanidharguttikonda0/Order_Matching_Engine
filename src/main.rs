use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use crossbeam_channel::bounded;
use matching_engine::{MatchingEngine, LogEvent}; // Import from your lib
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Creating the Channels
    // Channel 1: Gateway -> Engine.
    // We send a simple tuple: (is_buy, qty, price)
    // Capacity 1024 means we can buffer 1024 orders if the engine gets busy.
    // from now sending batch orders, if each vec of size 50 then 50*1024 approx we scaled to 50K orders
    let (tx_order, rx_order) = bounded::<Vec<(bool, u32, u64)>>(1024);

    // Channel 2: Engine -> Logger
    let (tx_log, rx_log) = bounded::<LogEvent>(1024);

    println!("🚀 Starting HFT Engine...");

    // 2. Spawn the Matching Engine (The Single-Threaded Beast)
    // 2. Spawn the Matching Engine
    thread::spawn(move || {
        let mut engine = MatchingEngine::new(tx_log);
        println!("✅ Engine Thread Started");

        // Receive a BATCH
        while let Ok(batch) = rx_order.recv() {
            // Process the whole batch hot
            for (is_buy, qty, price) in batch {
                if is_buy {
                    engine.buy_order(qty, price);
                } else {
                    engine.sell_order(qty, price);
                }
            }
        }
    });

    // 3. Spawn the Logger (The Slow Writer)
    thread::spawn(move || {
        while let Ok(event) = rx_log.recv() {
            // In a real job, you'd write to a file or Kafka here.
            // For now, print to screen so you can see it working.
            match event {
                LogEvent::OrderExecuted { price, qty, order_type } => {
                    println!("[TRADE] {:?} {} @ {}", order_type, qty, price);
                }
            }
        }
    });

    // 4. Start the TCP Gateway (The Front Door)
    // Inside main.rs

    // ... (Engine spawning code remains similar, see Step 3) ...

    // 4. Start the TCP Gateway
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("✅ Gateway listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("➕ New connection: {}", addr);

        // OPTIMIZATION 1: Disable Nagle's Algorithm
        // Essential for low latency!
        if let Err(e) = socket.set_nodelay(true) {
            println!("Failed to set nodelay: {}", e);
        }

        let tx = tx_order.clone();

        tokio::spawn(async move {
            // OPTIMIZATION 2: Read in 4KB Chunks
            let mut buf = [0u8; 4096];
            let mut remainder_buf = Vec::new(); // In case a packet gets cut in half

            loop {
                // We don't use read_exact anymore. We read whatever is available.
                match socket.read(&mut buf).await {
                    Ok(0) => return, // Connection closed
                    Ok(n) => {
                        let mut batch = Vec::with_capacity(50); // Pre-allocate batch

                        // Combine any leftover bytes from previous read with new data
                        let mut data_to_process = if !remainder_buf.is_empty() {
                            let mut v = remainder_buf.clone();
                            v.extend_from_slice(&buf[0..n]);
                            v
                        } else {
                            buf[0..n].to_vec()
                        };

                        remainder_buf.clear();

                        let mut i = 0;
                        while i + 13 <= data_to_process.len() {
                            // ZERO-COPY PARSING
                            let slice = &data_to_process[i..i+13];
                            let tag = slice[0];
                            let price = u64::from_be_bytes(slice[1..9].try_into().unwrap());
                            let qty = u32::from_be_bytes(slice[9..13].try_into().unwrap());

                            let is_buy = match tag {
                                b'B' => true,
                                b'S' => false,
                                _ => {
                                    i += 1; // Skip invalid byte and try to recover
                                    continue;
                                }
                            };

                            batch.push((is_buy, qty, price));
                            i += 13; // Move to next packet
                        }

                        // Save any leftover bytes (e.g., if we read half a packet)
                        if i < data_to_process.len() {
                            remainder_buf.extend_from_slice(&data_to_process[i..]);
                        }

                        // Send the entire batch to the engine in ONE channel operation
                        if !batch.is_empty() {
                            let _ = tx.send(batch);
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}