use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic;

/// Generates specific sequences of events.
/// Still very basic.
pub struct DatasetGenerator {
    num_features: usize,
    pub counter: atomic::AtomicI64,
    feature_names: Vec<String>,
}

type Datapoint = HashMap<String, Value>;
type Out = String;

impl DatasetGenerator {
    pub fn new(num_features: usize) -> DatasetGenerator {
        let mut feature_names = Vec::new();

        for i in 0..num_features {
            feature_names.push(format!("f_{}", i));
        }

        DatasetGenerator {
            num_features,
            counter: atomic::AtomicI64::new(0),
            feature_names,
        }
    }

    /// Generates the next item in the sequence (iterator-like).
    pub fn next(&self) -> Out {
        let value = self.counter.fetch_add(1, atomic::Ordering::SeqCst);
        self.gen(value)
    }

    /// Generates the ith item in the sequence.
    pub fn gen(&self, ith: i64) -> Out {
        let mut data = Datapoint::with_capacity(self.num_features);

        for f in 0..self.num_features {
            let name = self.feature_names.get(f).unwrap();
            data.insert(name.to_string(), Value::from(ith));
        }

        serde_json::json!(data).to_string()
    }
}
