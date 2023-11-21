use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Collection<T> {
    data: HashMap<String, T>,
}

impl<T> Collection<T> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

struct MEMOdb {
    version: &'static str,
    data: HashMap<String, Value>,
    path: &'static str,
}

impl MEMOdb {
    fn new() -> Self {
        Self {
            version: "v0.02",
            data: HashMap::new(),
            path: "./MEM.json",
        }
    }

    fn add_collection<T>(&mut self, name: &str, collection: Collection<T>) 
    where
        T: Serialize,
    {
        self.data.insert(name.to_string(), json!(collection));
    }

    fn get<T>(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }

    fn list(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    fn del_collection(&mut self, name: &str) {
        self.data.remove(name);
    }

    fn dump(&self) {
        let serialized = serde_json::to_string(&self.data).unwrap();
        let mut file = fs::File::create(self.path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn load(&mut self) {
        let file = fs::read_to_string(self.path).unwrap();
        self.data = serde_json::from_str(&file).unwrap();
    }
}