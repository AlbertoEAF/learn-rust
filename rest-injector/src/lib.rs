//! Lib!
//!

pub mod generator;

use serde::ser::{SerializeMap, Serializer};
//use std::vec::Vec;

extern crate owning_ref;
use owning_ref::BoxRef;

struct EventJSON<'a> {
    serializer: Option<Box<serde_json::ser::Serializer<Vec<u8>>>>,
    map: Option<Box<&'a serde_json::ser::Compound<'a, Vec<u8>, serde_json::ser::CompactFormatter>>>,
}

impl<'a> EventJSON<'a> {
    pub fn new(size: usize) -> (std::boxed::Box<serde_json::Serializer<std::vec::Vec<u8>>>, std::boxed::Box<serde_json::ser::Compound<'a, std::vec::Vec<u8>, serde_json::ser::CompactFormatter>>) {

        let mut serializer = Box::new(serde_json::ser::Serializer::new(Vec::new()));

        let mut map = Box::new(serializer.serialize_map(Some(size)).unwrap());

        (serializer, map)
        /*EventJSON {
            serializer: Some(serializer),
            map: Some(map),
        }*/
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
