extern crate threads_pool;

use std::collections::HashMap;
use serde_json::Value;
use std::sync::atomic;
use std::sync::Arc;

pub struct DatasetGenerator {
    num_features: usize,
    pub counter: atomic::AtomicI64,
    feature_names: Vec<String>,
}

type Datapoint = HashMap<String, Value>;

impl DatasetGenerator {
    pub fn new(num_features: usize) -> DatasetGenerator {
        let mut feature_names = Vec::new();

        for i in 0..num_features {
            feature_names.push(format!("f_{}", i));
        }

        DatasetGenerator { 
            num_features, 
            counter: atomic::AtomicI64::new(0), 
            feature_names 
        }
    }

    pub fn next(&mut self) -> i64 {
        self.counter.fetch_add(1, atomic::Ordering::SeqCst)
    }


    pub fn gen(&self, value: i64) -> String {

        let mut data = Datapoint::with_capacity(self.num_features.try_into().unwrap());
        for f in 0..self.num_features {
            //let name = format!("f_{}", f);                        
            let name = self.feature_names.get(f).unwrap();
            data.insert(name.to_string(), Value::from(value));            
        }
        serde_json::json!(data).to_string()
    }
}


//unsafe impl Send for DatasetGenerator {}
//unsafe impl Sync for DatasetGenerator {}