pub mod generator;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

/// Use a generator thread pool and send generated events through a channel to a consumer thread.
fn main() {
    const N: i64 = 25000;
    let (tx, rx) = mpsc::channel();

    let producer_thread = thread::spawn(move || {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(6)
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
        println!("  -» Consumed #{}:   {} ...     ", j, &s[..15]);
    }
    println!("  -» Consumer done!!");

    producer_thread.join().unwrap();

    println!("Success. Exit!");
}
