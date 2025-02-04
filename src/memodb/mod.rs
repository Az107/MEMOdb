// Written by Alberto Ruiz 2024-03-08
// MEMOdb is a in-memory database,
// it will store the data in memory and provide a simple API to interact with it
//
// The MEMOdb will have a collection of documents, each document will be a HashMap<String, DataType>

mod collection;
mod data_type;
pub use collection::{Collection, Document, DocumentJson};
pub use data_type::DataType;
use serde_json::Value;
use std::{fs, str::FromStr};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct MEMOdb {
    pub version: &'static str,
    pub path: String,
    collections: Vec<Collection>,
}

impl MEMOdb {
    pub fn new() -> Self {
        MEMOdb {
            version: VERSION,
            path: "./memo.json".to_string(),
            collections: Vec::new(),
        }
    }

    pub fn load(path: &str) -> Result<Self, &str> {
        let contents = fs::read_to_string(path);
        if contents.is_err() {
            return Err("Error reading file");
        }
        let contents = contents.unwrap();
        let mut collections = Vec::new();
        let json = Value::from_str(contents.as_str());
        if json.is_err() {
            return Err("Error parsing file");
        }
        let json = json.unwrap();
        let json = json.as_array();
        if json.is_none() {
            return Err("Error parsing file");
        }
        let json = json.unwrap();
        for coll in json {
            let coll = coll.to_string();
            let coll = coll.as_str();
            // let collection = Collection::from_json(coll);
            // if collection.is_err() {
            //     continue;
            // }
            // collections.push(collection.unwrap());
        }

        Ok(MEMOdb {
            version: VERSION,
            collections: collections,
            path: path.to_string(),
        })
    }

    pub fn dump(&self) -> Result<(), &str> {
        let mut list = Vec::new();

        for collection in self.collections.iter() {
            //list.push(collection.to_json_value());
        }

        let json = Value::Array(list);
        let json = json.to_string();
        let json = json.as_str();
        fs::write(self.path.as_str(), json);
        Ok(())
    }

    pub fn create_collection(&mut self, name: &str) -> Result<&mut Collection, &str> {
        //check if collection exists
        if self.collections.iter().any(|x| x.name == name) {
            Err("Collection already exists")
        } else {
            let collection = Collection::new(name);
            self.collections.push(collection);
            return Ok(self.collections.last_mut().unwrap());
        }
    }

    pub fn get_collection(&mut self, name: &str) -> Option<&mut Collection> {
        //return a mutable reference to collection
        self.collections
            .iter_mut()
            .find(|x| x.name == name.to_string())
    }

    pub fn get_collection_list(&self) -> Vec<String> {
        let mut collection_list: Vec<String> = Vec::new();
        for collection in self.collections.iter() {
            collection_list.push(collection.name.clone());
        }
        collection_list
    }

    pub fn remove_collection(&mut self, name: String) -> Collection {
        let index = self
            .collections
            .iter()
            .position(|x| x.name == name)
            .unwrap();
        self.collections.remove(index)
    }
}

//TEST
#[cfg(test)]
#[test]
fn test_memodb() {
    let mut memodb = MEMOdb::new();
    let r1 = memodb.create_collection("users").is_ok();
    let r2 = memodb.create_collection("posts").is_ok();
    assert!(r1);
    assert!(r2);
    assert_eq!(memodb.collections.len(), 2);
    assert_eq!(memodb.collections[0].name, "users");
    assert_eq!(memodb.collections[1].name, "posts");
    assert_eq!(memodb.get_collection("users").unwrap().name, "users");
    assert_eq!(memodb.get_collection("posts").unwrap().name, "posts");
    assert_eq!(memodb.get_collection_list().len(), 2);
    assert_eq!(memodb.remove_collection("users".to_string()).name, "users");
    assert_eq!(memodb.collections.len(), 1);
    assert_eq!(memodb.remove_collection("posts".to_string()).name, "posts");
    assert_eq!(memodb.collections.len(), 0);
}

#[test]
fn add_document() {
    let mut memodb = MEMOdb::new();
    let _ = memodb.create_collection("users");
    let collection = memodb.get_collection("users").unwrap();
    let id1 = collection.add("John", doc! {"name" => "John", "age" => 30});
    let id2 = collection.add("Jane", doc! {"name" => "Jane", "age" => 25});
    assert_eq!(collection.count(), 2);
    let document = collection.get("John").unwrap();
}

//
