extern crate threads_pool;

use threads_pool::*;

mod generator;

use std::time::Instant;
use std::sync::mpsc;
use std::thread;
use std::sync::Arc;

fn main() {

    let pool = ThreadPool::new(4);

    let mut generator = Arc::new(generator::data_generator::DatasetGenerator::new(3000));

    const N: i64 = 10000;
    let (tx, rx) = mpsc::channel();

    for i in 0..N {
        
        let tx = tx.clone();

        generator.clone().counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let gen = generator.clone();
        
        //let p = generator.next();
        pool.execute(move || {
            tx.send(gen.gen(i)).unwrap();                     
        }).unwrap();
        
    }

    for _ in 0..N {
        println!("{}", rx.recv().unwrap());
    }
    
    
 
}
