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

    pub fn createCollection(&mut self, name: String) {
        let collection = Collection::new(name);
        self.collections.push(collection);
    }

    pub fn getCollection(&self, name: String) -> Option<&Collection> {
        self.collections.iter().find(|&x| x.name == name)
    }

    pub fn getAllCollections(&self) -> &Vec<Collection> {
        &self.collections
    }

    pub fn getCollectionList(&self) -> Vec<String> {
        let mut collectionList: Vec<String> = Vec::new();
        for collection in self.collections.iter() {
            collectionList.push(collection.name.clone());
        }
        collectionList
    }

    fn removeCollection(&mut self, name: String) -> Collection {
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
        memodb.createCollection("users".to_string());
        memodb.createCollection("posts".to_string());
        assert_eq!(memodb.collections.len(), 2);
        assert_eq!(memodb.collections[0].name, "users");
        assert_eq!(memodb.collections[1].name, "posts");
        assert_eq!(memodb.getCollection("users".to_string()).unwrap().name, "users");
        assert_eq!(memodb.getCollection("posts".to_string()).unwrap().name, "posts");
        assert_eq!(memodb.getAllCollections().len(), 2);
        assert_eq!(memodb.removeCollection("users".to_string()).name, "users");
        assert_eq!(memodb.collections.len(), 1);
        assert_eq!(memodb.removeCollection("posts".to_string()).name, "posts");
        assert_eq!(memodb.collections.len(), 0);
    }
}