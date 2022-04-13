//! Lib!
//!

pub mod generator;

use serde::ser::{SerializeMap, Serializer};
//use std::vec::Vec;

struct EventJSON {
    serializer: Option<serde_json::ser::Serializer<Vec<u8>>>,
    map: Option<serde_json::ser::Compound<'static, Vec<u8>, serde_json::ser::CompactFormatter>>,
}

impl EventJSON {
    pub fn new(size: usize) -> EventJSON {
        let mut serializer = serde_json::ser::Serializer::new(Vec::new());
        let mut map = serializer.serialize_map(Some(size)).unwrap();

        EventJSON {
            serializer: Some(serializer),
            map: Some(map),
        }
    }

    pub fn insert(&mut self, k: &str, v: &str) {
        if let Some(map) = &mut self.map {
            map.serialize_entry(k, v);
        } else {
            panic!("Already serialized! Unusable object!");
        }
    }

    pub fn serialize(&mut self) -> String {
        if let Some(serializer) = self.serializer.take() {
            String::from_utf8(serializer.into_inner()).expect("Invalid UTF-8.")
        } else {
            panic!("Already serialized! Unusable object!");
        }
    }
}
