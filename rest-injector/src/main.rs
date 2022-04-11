extern crate threads_pool;

use threads_pool::*;

pub mod generator;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

/// Use a generator thread pool and send generated events through a channel to a consumer thread.
fn main() {
    const N: i64 = 12;  // Increase this if you're not getting the deadlock yet, or run cargo run again until it happens.
    let (tx, rx) = mpsc::channel();

    let producer_thread = thread::spawn(move || {
        let mut pool = ThreadPool::new(4);

        let generator = Arc::new(generator::data_generator::DatasetGenerator::new(3000));
        for i in 0..N {
            println!("> Generate #{}", i);       
            let tx_ref = tx.clone();
            let generator_ref = generator.clone();
            pool.execute(move || {
                println!("> Generate #{}: Thread executed!", i);
                tx_ref.send(generator_ref.next()).expect("tx failed.");              // This fails!
                //tx_ref.send(format!(" {}            ", i)).expect("tx failed.");   // This works!
            })
            .unwrap();
        }        
        pool.close();
        println!("Generator done!");
    });

    println!("  -» Consumer consuming!");
    for j in 0..N {
        let s = rx.recv().expect("rx failed");
        println!("  -» Consumed #{}:   {} ...     ", j, &s[..10]);
    }
    println!("  -» Consumer done!!");

    producer_thread.join().unwrap();
    
    println!("Success. Exit!");
}
