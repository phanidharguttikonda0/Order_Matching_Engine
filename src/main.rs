

use std::collections::{BTreeMap, VecDeque};
use std::io;
use matching_engine::{buy_order, sell_order, Order, order_book};




fn main() {
    println!("Welcome to the Stock Market Exchange");
    // need to take the input from the user
    let mut order = String::new() ;
    let mut buy_orders: order_book = BTreeMap::new() ;
    let mut sell_orders: order_book = BTreeMap::new() ;
    let mut buy_count: u64 = 0 ;
    let mut sell_count: u64 = 0 ;
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
            buy_order(quantity, price, &mut buy_orders, &mut sell_orders, buy_count+1) ;
            buy_count += 1 ;
        }else if order_type == "sell" {
            let quantity: u32 = order_result[1].parse().unwrap() ;
            let price: f32 = order_result[2].split("\n").collect::<Vec<&str>>()[0].parse().unwrap() ;
            let price: u64 = (price * 100.0 )as u64;
            sell_order(quantity, price, &mut buy_orders, &mut sell_orders, sell_count+1) ;
            sell_count += 1 ;
        }else {
            println!("--------------------------------------------") ;
            println!("here is the Buy Orders") ;
            println!("{:?}", buy_orders) ;
            println!("---------------------------------------------") ;
            println!("here is the Sell Orders") ;
            println!("{:?}", sell_orders) ;
        }
        order = "".to_string() ;
    }
}





