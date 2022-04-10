extern crate threads_pool;

use threads_pool::*;

mod generator;

use std::time::Instant;
use std::sync::mpsc::channel;
use std::thread;

fn main() {

    let pool = ThreadPool::new(4);

    let mut generator = generator::data_generator::DatasetGenerator::new(3000);

    let N = 1000;
    let (tx, rx) = channel();

    for _ in 0..N {
        
        unsafe {
        //let p = generator.next();
            pool.execute(move || {
                tx.clone().send(generator.next()).unwrap();           
            });
        }
    }

    for _ in 0..N {
        println!("{}", rx.recv().unwrap());
    }
    
    
 
}
