use std::collections::{BTreeMap};
use crossbeam_channel::Sender;

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub quantity: u32,
    pub price: u64,
    pub next: Option<usize> // The "Pointer"
}

pub type OrderBook = BTreeMap<u64, (usize, usize)>; // (Head, Tail)

#[derive(Debug, Clone, Copy)]
pub enum OrderType { Buy, Sell }

pub enum LogEvent {
    OrderExecuted { price: u64, qty: u32, order_type: OrderType }
}

pub struct MatchingEngine {
    pub buy_order_book: OrderBook,
    pub sell_order_book: OrderBook,
    pub buy_orders_count: u64,
    pub sell_orders_count: u64,
    pub log_sender: Sender<LogEvent>,
    pub orders: Vec<Order>,      // The Arena
    pub free_spots: Vec<usize>   // The Recycle Bin
}

impl MatchingEngine {
    pub fn new(log_sender: Sender<LogEvent>) -> MatchingEngine {
        MatchingEngine {
            buy_orders_count: 0,
            sell_orders_count: 0,
            buy_order_book: BTreeMap::new(),
            sell_order_book: BTreeMap::new(),
            log_sender,
            orders: Vec::with_capacity(100_000), // PRE-ALLOCATE! Critical for speed
            free_spots: Vec::new()
        }
    }

    // --- Helper to manage Arena memory ---
    fn allocate_order(&mut self, order: Order) -> usize {
        if let Some(index) = self.free_spots.pop() {
            self.orders[index] = order;
            index
        } else {
            let index = self.orders.len();
            self.orders.push(order);
            index
        }
    }

    pub fn buy_order(&mut self, mut quantity: u32, price: u64) {
        self.buy_orders_count += 1;

        // 1. Check Sells (Lowest Price First)
        // We collect keys to avoid borrowing self.sell_order_book while mutating self.orders
        let prices: Vec<u64> = self.sell_order_book
            .range(..=price)
            .map(|(p, _)| *p)
            .collect();

        for p in prices {
            if quantity == 0 { break; }

            // Inner Loop: Consume the Linked List at this price
            loop {
                // Get Head Index safely
                let head_index = match self.sell_order_book.get(&p) {
                    Some(&(head, _)) => head,
                    None => break, // Price level exhausted
                };

                // Access the Order in Arena
                // We use a block to limit the borrow scope of 'order'
                let (should_remove, next_ptr, matched_qty) = {
                    let order = &mut self.orders[head_index];

                    let trade_qty = std::cmp::min(quantity, order.quantity);
                    quantity -= trade_qty;
                    order.quantity -= trade_qty;

                    // Send Log (Non-blocking)
                    let _ = self.log_sender.try_send(LogEvent::OrderExecuted {
                        qty: trade_qty, price: p, order_type: OrderType::Buy
                    });

                    // Return data to update state outside the borrow
                    (order.quantity == 0, order.next, trade_qty)
                };

                // Cleanup Logic
                if should_remove {
                    // 1. Recycle the spot
                    self.free_spots.push(head_index);

                    // 2. Move Head Pointer
                    if let Some(next_index) = next_ptr {
                        // Update Book to point to next
                        if let Some(entry) = self.sell_order_book.get_mut(&p) {
                            entry.0 = next_index;
                        }
                    } else {
                        // No next item, remove price level entirely
                        self.sell_order_book.remove(&p);
                        break; // Move to next price
                    }
                } else {
                    // Order not filled (Buyer ran out of qty), stop everything
                    break;
                }

                if quantity == 0 { break; }
            }
        }

        // 2. If Quantity Remains, Add to Buy Book
        if quantity > 0 {
            let order = Order {
                id: self.buy_orders_count,
                quantity,
                price,
                next: None
            };

            let index = self.allocate_order(order);

            // Link it
            self.buy_order_book.entry(price)
                .and_modify(|(_, tail)| {
                    // Update OLD tail to point to NEW index
                    self.orders[*tail].next = Some(index);
                    // Update tail to be NEW index
                    *tail = index;
                })
                .or_insert((index, index));
        }
    }

    // Mirror logic for sell_order...
    pub fn sell_order(&mut self, mut quantity: u32, price: u64) {
        self.sell_orders_count += 1;

        // Reverse iterator for Bids (Highest First)
        let prices: Vec<u64> = self.buy_order_book
            .range(price..)
            .rev() // <--- CRITICAL
            .map(|(p, _)| *p)
            .collect();

        for p in prices {
            if quantity == 0 { break; }

            loop {
                let head_index = match self.buy_order_book.get(&p) {
                    Some(&(head, _)) => head,
                    None => break,
                };

                let (should_remove, next_ptr, _) = {
                    let order = &mut self.orders[head_index];
                    let trade_qty = std::cmp::min(quantity, order.quantity);
                    quantity -= trade_qty;
                    order.quantity -= trade_qty;

                    let _ = self.log_sender.try_send(LogEvent::OrderExecuted {
                        qty: trade_qty, price: p, order_type: OrderType::Sell
                    });

                    (order.quantity == 0, order.next, trade_qty)
                };

                if should_remove {
                    self.free_spots.push(head_index);
                    if let Some(next_index) = next_ptr {
                        if let Some(entry) = self.buy_order_book.get_mut(&p) {
                            entry.0 = next_index;
                        }
                    } else {
                        self.buy_order_book.remove(&p);
                        break;
                    }
                } else {
                    break;
                }
                if quantity == 0 { break; }
            }
        }

        if quantity > 0 {
            let order = Order {
                id: self.sell_orders_count,
                quantity,
                price,
                next: None
            };
            let index = self.allocate_order(order);
            self.buy_order_book.entry(price)
                .and_modify(|(_, tail)| {
                    self.orders[*tail].next = Some(index);
                    *tail = index;
                })
                .or_insert((index, index));
        }
    }
}