use collection::{Collection, Document, DataType};

use crate::collection;

pub struct MEMOdb {
    collections: Vec<Collection>,
}

impl MEMOdb {
    fn new() -> Self {
        MEMOdb {
            collections: Vec::new(),
        }
    }

    fn createCollection(&mut self, name: String) {
        let collection = Collection::new(name);
        self.collections.push(collection);
    }

    fn getCollection(&self, name: String) -> Option<&Collection> {
        self.collections.iter().find(|&x| x.name == name)
    }

    fn getAllCollections(&self) -> &Vec<Collection> {
        &self.collections
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