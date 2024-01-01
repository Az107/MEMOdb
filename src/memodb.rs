use collection::{Collection, Document, DataType};

use crate::collection;

pub struct MEMOdb {
    pub version: &'static str,
    collections: Vec<Collection>,
}

impl MEMOdb {
    pub fn new() -> Self {
        MEMOdb {
            version: "0.1.0",
            collections: Vec::new(),
        }
    }

    pub fn create_collection(&mut self, name: String) {
        let collection = Collection::new(name);
        self.collections.push(collection);
    }

    pub fn get_collection(&mut self, name: String) -> Option<&mut Collection> {
        //return a mutable reference to collection
        self.collections.iter_mut().find(|x| x.name == name)
    }

    pub fn get_all_collections(&self) -> &Vec<Collection> {
        &self.collections
    }

    pub fn get_collection_list(&self) -> Vec<String> {
        let mut collection_list: Vec<String> = Vec::new();
        for collection in self.collections.iter() {
            collection_list.push(collection.name.clone());
        }
        collection_list
    }

    fn remove_collection(&mut self, name: String) -> Collection {
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
mod tests {

    #[test]
    fn test_memodb() {
        let mut memodb = crate::memodb::MEMOdb::new();
        memodb.create_collection("users".to_string());
        memodb.create_collection("posts".to_string());
        assert_eq!(memodb.collections.len(), 2);
        assert_eq!(memodb.collections[0].name, "users");
        assert_eq!(memodb.collections[1].name, "posts");
        assert_eq!(memodb.get_collection("users".to_string()).unwrap().name, "users");
        assert_eq!(memodb.get_collection("posts".to_string()).unwrap().name, "posts");
        assert_eq!(memodb.get_all_collections().len(), 2);
        assert_eq!(memodb.remove_collection("users".to_string()).name, "users");
        assert_eq!(memodb.collections.len(), 1);
        assert_eq!(memodb.remove_collection("posts".to_string()).name, "posts");
        assert_eq!(memodb.collections.len(), 0);
    }
}