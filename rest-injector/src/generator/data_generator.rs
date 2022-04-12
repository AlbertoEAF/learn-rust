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

/*
struct EventJSON {
    serializer: Option<serde_json::ser::Serializer<Vec<u8>>>,
    map: Option<
        serde_json::ser::Compound<'static, std::vec::Vec<u8>, serde_json::ser::CompactFormatter>,
    >,
}

impl EventJSON {
    pub fn new(size: usize) -> EventJSON {

        let mut serializer = serde_json::ser::Serializer::new(Vec::new());
        let map = Some(serializer.serialize_map(Some(size)).unwrap());

        EventJSON {
            serializer: None,
            map: map,
        }
    }

    pub fn insert(&mut self, k: &str, v: &str) {
        if let Some(map) = &mut self.map {
            map.serialize_entry(k, v);
        } else {
            panic!("Already serialized! Unusable object!");
        }
    }

    pub fn to_string(&mut self) -> String {
        if let Some(serializer) = self.serializer.take() {
            String::from_utf8(serializer.into_inner()).expect("Invalid UTF-8.")
        } else {
            panic!("Already serialized! Unusable object!");
        }
    }
}
*/

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

        let mut serializer = serde_json::ser::Serializer::new(Vec::new());
        let mut map = serializer.serialize_map(Some(self.num_features)).unwrap();
        //let mut ev = EventJSON::new(self.num_features);

        for f in 0..self.num_features {
            let name = self.feature_names.get(f).unwrap();

            map.serialize_entry(name, &ith.to_string())
                .expect("Can't serialize feature {}.");
            //ev.insert(name, &ith.to_string());
        }
        map.end().expect("Serialization failed.");

        String::from_utf8(serializer.into_inner()).expect("Invalid UTF-8.")
        //ev.to_string()
    }
}
