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
    let (tx_order, rx_order) = bounded::<(bool, u32, u64)>(1024);

    // Channel 2: Engine -> Logger
    let (tx_log, rx_log) = bounded::<LogEvent>(1024);

    println!("🚀 Starting HFT Engine...");

    // 2. Spawn the Matching Engine (The Single-Threaded Beast)
    thread::spawn(move || {
        let mut engine = MatchingEngine::new(tx_log);
        println!("✅ Engine Thread Started");

        // This loop runs at nanosecond speed
        while let Ok((is_buy, qty, price)) = rx_order.recv() {
            if is_buy {
                engine.buy_order(qty, price);
            } else {
                engine.sell_order(qty, price);
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
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("✅ Gateway listening on 127.0.0.1:8080");

    loop {
        // Accept a new connection (e.g., a new Trader connecting)
        let (mut socket, addr) = listener.accept().await?;
        println!("➕ New connection: {}", addr);

        // Clone the sender so this connection can talk to the engine
        let tx = tx_order.clone();

        // Spawn a lightweight Tokio task for this connection
        tokio::spawn(async move {
            let mut buf = [0u8; 13]; // Packet Size: 1 (Tag) + 8 (Price) + 4 (Qty) = 13 Bytes

            loop {
                // Read exactly 13 bytes.
                // "read_exact" is crucial. It waits until we have the full packet.
                match socket.read_exact(&mut buf).await {
                    Ok(_) => {
                        // ZERO-COPY PARSING (The 40 LPA Skill)
                        // We interpret raw bytes directly as numbers. No Strings.

                        let tag = buf[0]; // 'B' or 'S'

                        // Parse Price (Next 8 bytes)
                        let price = u64::from_be_bytes(buf[1..9].try_into().unwrap());

                        // Parse Qty (Next 4 bytes)
                        let qty = u32::from_be_bytes(buf[9..13].try_into().unwrap());

                        let is_buy = match tag {
                            b'B' => true,
                            b'S' => false,
                            _ => {
                                println!("❌ Invalid Packet Tag: {}", tag);
                                continue;
                            }
                        };

                        // Send to the Engine
                        // This takes microseconds.
                        if let Err(e) = tx.send((is_buy, qty, price)) {
                            println!("❌ Engine Dead: {}", e);
                            return;
                        }
                    }
                    Err(_) => {
                        // Connection closed or error
                        return;
                    }
                }
            }
        });
    }
}