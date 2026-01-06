use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Exchange...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected!");

    // Create a "Buy 10 @ 100" packet
    // Packet Structure: [Tag: 1 byte] [Price: 8 bytes] [Qty: 4 bytes]
    let mut buy_packet = Vec::new();
    buy_packet.push(b'B');              // Tag 'B'
    buy_packet.extend_from_slice(&100u64.to_be_bytes()); // Price 100
    buy_packet.extend_from_slice(&10u32.to_be_bytes());  // Qty 10

    // Create a "Sell 10 @ 100" packet (Matches immediately)
    let mut sell_packet = Vec::new();
    sell_packet.push(b'S');             // Tag 'S'
    sell_packet.extend_from_slice(&100u64.to_be_bytes());
    sell_packet.extend_from_slice(&10u32.to_be_bytes());

    // Blast 1000 orders
    for i in 0..1000 {
        stream.write_all(&buy_packet).await?;
        stream.write_all(&sell_packet).await?;

        if i % 100 == 0 {
            println!("Sent {} pairs...", i);
        }
        // Small sleep to not crash your console with logs,
        // remove this for full speed test!
        tokio::time::sleep(Duration::from_micros(100)).await;
    }

    println!("Done!");
    Ok(())
}