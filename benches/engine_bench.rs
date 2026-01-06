use criterion::{criterion_group, criterion_main, Criterion};
use matching_engine::{MatchingEngine, LogEvent};
use crossbeam_channel::bounded;

fn benchmark_orders(c: &mut Criterion) {
    c.bench_function("buy_sell_cycle", |b| {
        b.iter(|| {
            // 1. Create a "Black Hole" channel
            // We give it a huge capacity so it never blocks the engine
            let (tx, _rx) = bounded::<LogEvent>(1_000_000);

            // 2. Initialize Engine with the sender
            let mut engine = MatchingEngine::new(tx);

            // 3. Run the loop (100 pairs = 200 orders per iter)
            for _ in 0..100 {
                let price = 10000; // 100.00
                let quantity = 10;

                // Place a Buy
                engine.buy_order(quantity, price);

                // Place a Sell (Matches immediately)
                engine.sell_order(quantity, price);
            }
        })
    });
}

criterion_group!(benches, benchmark_orders);
criterion_main!(benches);