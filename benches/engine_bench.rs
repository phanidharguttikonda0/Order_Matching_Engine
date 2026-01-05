use criterion::{criterion_group, criterion_main, Criterion};
use matching_engine::{buy_order, order_book, sell_order}; // Imports your functions
use std::collections::BTreeMap;

fn benchmark_orders(c: &mut Criterion) {
    c.bench_function("buy_sell_cycle", |b| {
        b.iter(|| {
            // 1. Setup the data structures (Just like in your main)
            let mut buy_orders: order_book = BTreeMap::new();
            let mut sell_orders: order_book = BTreeMap::new();

            // 2. Run a loop of interactions
            // We simulate 100 orders to get a stable average
            for i in 0..100 {
                let price = 10000; // 100.00
                let quantity = 10;
                let id = i as u64;

                // Place a Buy
                buy_order(quantity, price, &mut buy_orders, &mut sell_orders, id);

                // Place a Sell (Matches immediately)
                sell_order(quantity, price, &mut buy_orders, &mut sell_orders, id);
            }
        })
    });
}

criterion_group!(benches, benchmark_orders);
criterion_main!(benches);