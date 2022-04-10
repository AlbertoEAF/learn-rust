extern crate threads_pool;

use threads_pool::*;

mod generator;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

fn main() {
    const N: i64 = 12;  // Increase this if you're not getting the deadlock yet, or run cargo run again until it happens.
    let (tx, rx) = mpsc::channel();

    let tx_producer = tx.clone();
    let producer_thread = thread::spawn(move || {
        let pool = ThreadPool::new(4);
        let generator = Arc::new(generator::data_generator::DatasetGenerator::new(3000));
        for i in 0..N {
            println!("Generating #{}", i);
            let tx_ref = tx_producer.clone();
            let generator_ref = generator.clone();
            pool.execute(move || {
                tx_ref.send(generator_ref.next()).expect("tx failed.");              // This fails!
                //tx_ref.send(format!(" {}            ", i)).expect("tx failed.");   // This works!
            })
            .unwrap();
        }

        println!("Generator done!");
    });

    println!("-» Consumer consuming!");
    for j in 0..N {
        let s = rx.recv().expect("rx failed");
        println!("-» Consumed #{}:   {} ...     ", j, &s[..20]);
    }
    println!("Consumer done!!");

    producer_thread.join().unwrap();
    println!("Success. Exit!");
}
