use std::{io, thread};
use crossbeam_channel::bounded;
use matching_engine::{LogEvent, MatchingEngine};




fn main() {
    println!("Welcome to the Stock Market Exchange");
    // need to take the input from the user
    let (sender, receiver) = bounded::<LogEvent>(1024);

    thread::spawn(move || {
        // This loop runs forever in the background
        while let Ok(event) = receiver.recv() {
            match event {
                LogEvent::OrderPlaced { id, price, order_type, quantity } => {
                    println!("[LOG] {:?} Order #{} Placed: {} @ {}", order_type, id, quantity, price);
                }
                LogEvent::OrderExecuted { price, qty, remaining_quantity, order_type } => {
                    println!("[LOG] ⚡ {:?} TRADE! Sold {} @ {} and remaining quantity to be executed was {}", order_type, qty, price, remaining_quantity);
                }
            }
        }
    });

    let mut matching_engine = MatchingEngine::new(sender) ;
    let mut order = "".to_string();
    loop {
        println!("place an Order") ;
        io::stdin().read_line(&mut order).expect("failed to read input") ;
        // buy quantity price
        let order_result = order.split(" ").collect::<Vec<&str>>() ;
        println!("order result was {:?}", order_result) ;
        let order_type = order_result[0].to_lowercase() ;
        println!("order type {}", order_type) ;
        if order_type == "buy" {
            let quantity: u32 = order_result[1].parse().unwrap() ;
            let price: f32 = order_result[2].split("\n").collect::<Vec<&str>>()[0].parse().unwrap() ;
            let price: u64 = (price * 100.0 )as u64;
            matching_engine.buy_order(quantity, price) ;
        }else if order_type == "sell" {
            let quantity: u32 = order_result[1].parse().unwrap() ;
            let price: f32 = order_result[2].split("\n").collect::<Vec<&str>>()[0].parse().unwrap() ;
            let price: u64 = (price * 100.0 )as u64;
            matching_engine.sell_order(quantity, price) ;
        }else {
            println!("--------------------------------------------") ;
            println!("here is the Buy Orders") ;
            println!("{:?}", matching_engine.buy_order_book) ;
            println!("---------------------------------------------") ;
            println!("here is the Sell Orders") ;
            println!("{:?}", matching_engine.sell_order_book) ;
        }
        order = "".to_string() ;
    }
}





