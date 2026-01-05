

use std::collections::{BTreeMap, VecDeque};
use std::io;
use matching_engine::{MatchingEngine};




fn main() {
    println!("Welcome to the Stock Market Exchange");
    // need to take the input from the user
    let mut matching_engine = MatchingEngine::new() ;
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





