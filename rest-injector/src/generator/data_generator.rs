use serde::ser::{SerializeMap, Serializer};
use std::sync::atomic;

/// Generates specific sequences of events.
/// Still very basic.
pub struct DatasetGenerator {
    num_features: usize,
    pub counter: atomic::AtomicI64,
    feature_names: Vec<String>,
}

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
        //let mut ev = EventJSON::new(self.num_features);
        let mut serializer = serde_json::ser::Serializer::new(Vec::new());
        let mut map = serializer.serialize_map(Some(self.num_features)).unwrap();

        for f in 0..self.num_features {
            let name = self.feature_names.get(f).unwrap();

            //ev.insert(name, &ith.to_string());
            map.serialize_entry(name, &ith.to_string())
                .expect("Can't serialize feature {}.");
        }
        map.end().expect("Serialization failed.");

        //ev.serialize()
        String::from_utf8(serializer.into_inner()).expect("Invalid UTF-8.")
    }
}
