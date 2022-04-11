pub mod generator;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

/// Use a generator thread pool and send generated events through a channel to a consumer thread.
fn main() {
    const N: i64 = 25000; // Increase this if you're not getting the deadlock yet, or run cargo run again until it happens.
    let (tx, rx) = mpsc::sync_channel(1000);

    let producer_thread = thread::spawn(move || {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();

        let generator = Arc::new(generator::data_generator::DatasetGenerator::new(3000));
        for _ in 0..N {
            let tx_ref = tx.clone();
            let generator_ref = generator.clone();
            pool.spawn(move || {
                tx_ref.send(generator_ref.next()).expect("tx failed.");
            });
        }

        println!("Generator done!");
    });

    for (j, s) in rx.iter().enumerate() {
        println!("  -» Consumed #{}:   {} ...     ", j, &s[..10]);
    }
    println!("  -» Consumer done!!");

    producer_thread.join().unwrap();

    println!("Success. Exit!");
}
