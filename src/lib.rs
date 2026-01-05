use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64, // starting with 's' is a sell order , 'b' is a buy order .
    pub quantity: u32,
    pub price: u64
}


pub type order_book = BTreeMap<u64, VecDeque<Order>>;

pub fn buy_order(quantity: u32, price: u64, buy_orders: &mut order_book, sell_orders: &mut order_book, id: u64) {
    // we need to check whether there is an order matching the following price
    // we need to check for a sell_order, whether it exists or not, if exists we need to match with the first order
    let mut order = Order{
        id, // we are going to make random later
        quantity,
        price
    } ;

    /*
        we need to figure out least value order and then from that point we need to loop through each and
        every price point, so sell the Order
    */
    let price_listing_orders: Vec<u64> =
        sell_orders.range(..=price).map(|val| val.0.clone()).collect();



    if price_listing_orders.len() > 0 {
        let mut remaining_required_stocks = quantity as i64 ;
        let mut remove_prices = vec![] ;
        for price in price_listing_orders.iter() {

            // now we are going to match the orders
            let mut iterate = true ;

            let order_queue = sell_orders.get_mut(price).unwrap();
            while iterate {
                remaining_required_stocks = remaining_required_stocks - order_queue[0].quantity as i64 ;
                if remaining_required_stocks <= 0 {
                    if remaining_required_stocks == 0 {
                        order_queue.pop_front().unwrap() ;
                        if order_queue.len() == 0 {
                            remove_prices.push(price) ;
                        }
                    }else {
                        order_queue[0].quantity = (remaining_required_stocks * -1) as u32 ;
                    }
                    println!("Buy Order Matched Successfully") ;
                    iterate = false ;
                }else{
                    order_queue.pop_front().unwrap() ;
                    if order_queue.len() == 0 {
                        remove_prices.push(price) ;
                        iterate = false ;
                    }
                }
            }

            if remaining_required_stocks == 0 {
                break;
            }
        }
        if remaining_required_stocks > 0 {
            order.quantity = remaining_required_stocks as u32 ;
            //println!("remaining required quantity was {}", order.quantity) ;
            let mut value = VecDeque::new() ;
            value.push_back(order) ;
            buy_orders.insert(price, value) ;
        }

        for price in remove_prices {
            sell_orders.remove(price).unwrap() ;
        }
    }else {
        // println!("no order exists, so storing the order and executing when ever an order matches") ;
        let mut val: VecDeque<Order> = VecDeque::new() ;
        val.push_back(order) ;
        buy_orders.insert(price, val) ;
        println!("added to buy orders") ;
    }
}

pub fn sell_order(quantity: u32, price: u64, buy_orders: &mut order_book, sell_orders: &mut order_book, id: u64) {
    let mut order = Order{
        id, // we are going to make random later
        quantity,
        price
    } ;

    let price_listing_orders: Vec<u64> = buy_orders.range(price..).map(
        |value| value.0.clone()
    ).collect() ;

    if price_listing_orders.len() > 0  {
        let mut remove_prices = vec![] ;
        let mut remaining_stocks = quantity as i64;

        for price in price_listing_orders.iter().rev(){

            let mut iterate = true ;
            let order_queue = buy_orders.get_mut(price).unwrap() ;

            while iterate {
                remaining_stocks =  remaining_stocks - order_queue[0].quantity as i64 ;
                if remaining_stocks <= 0 {
                    if remaining_stocks == 0 {
                        // we are removing the buy order
                        order_queue.pop_front().unwrap() ;
                        if order_queue.len() == 0 {
                            remove_prices.push(price) ;
                        }
                    }else {
                        order_queue[0].quantity = (remaining_stocks * -1) as u32 ;
                    }
                    println!("Completed Order Matching Sell Order Executed Successfully") ;
                    iterate = false ;
                }else{
                    order_queue.pop_front().unwrap() ;
                    if order_queue.len() == 0 {
                        remove_prices.push(price) ;
                        iterate = false ;
                    }
                }
            }
            if remaining_stocks == 0 {
                break;
            }

        }

        if remaining_stocks > 0 {
            order.quantity = remaining_stocks as u32 ;
            let mut val = VecDeque::new() ;
            val.push_back(order) ;
            sell_orders.insert(price, val) ;
        }
       // println!("let's print remove prices {:#?}", remove_prices) ;
        for price in remove_prices.iter() {
            // println!("let's print buy_orders {:#?}", buy_orders) ;
            buy_orders.remove(price) .unwrap() ;
        }
    }else{
        // println!("No Orders Matched adding to Order Book") ;
        let mut val: VecDeque<Order> = VecDeque::new() ;
        val.push_back(order) ;
        sell_orders.insert(price, val) ;
        println!("added to buy orders") ;
    }
}