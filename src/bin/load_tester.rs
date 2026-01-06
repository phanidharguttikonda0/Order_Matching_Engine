use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_orders = 10_00_000;
    println!("Connecting to Gateway...");

    // 1. Connect
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    // Essential for benchmarking: Disable Nagle's algorithm on the client too
    stream.set_nodelay(true)?;
    println!("🚀 Starting Load Test: Sending {} orders...", total_orders);

    // 2. Prepare Data (Pre-calculate to measure ONLY network speed)
    let mut packet = Vec::new();
    packet.push(b'B'); // Tag
    packet.extend_from_slice(&100u64.to_be_bytes()); // Price
    packet.extend_from_slice(&10u32.to_be_bytes());  // Qty

    // We create a giant buffer of 100,000 orders to send as fast as possible
    // This ensures the CLIENT isn't the bottleneck.
    let mut giant_payload = Vec::with_capacity(packet.len() * total_orders);
    for _ in 0..total_orders {
        giant_payload.extend_from_slice(&packet);
    }

    // 3. Start Timer
    let start = Instant::now();

    // 4. Blast Data
    stream.write_all(&giant_payload).await?;

    // 5. Stop Timer
    let duration = start.elapsed();

    println!("✅ Done!");
    println!("------------------------------------------------");
    println!("Time Taken:      {:.2?}", duration);
    println!("Throughput:      {:.0} orders/sec", total_orders as f64 / duration.as_secs_f64());
    println!("------------------------------------------------");

    Ok(())
}