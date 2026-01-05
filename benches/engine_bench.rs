use criterion::{criterion_group, criterion_main, Criterion};
use matching_engine::{MatchingEngine}; // Imports your functions
use crossbeam_channel::bounded;

fn benchmark_orders(c: &mut Criterion) {
    c.bench_function("buy_sell_cycle", |b| {
        b.iter(|| {
            // 1. Setup the data structures (Just like in your main)
            let (tx, _rx) = bounded(100_000);
            let mut matching_engine = MatchingEngine::new(tx) ;

            // 2. Run a loop of interactions
            // We simulate 1000 orders to get a stable average
            for _ in 0..100 {
                let price = 10000; // 100.00
                let quantity = 10;

                // Place a Buy
                matching_engine.buy_order(quantity, price) ;

                // Place a Sell (Matches immediately)
                matching_engine.sell_order(quantity, price) ;
            }
        })
    });
}

criterion_group!(benches, benchmark_orders);
criterion_main!(benches);