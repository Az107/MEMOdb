use std::collections::HashMap;

#[derive(PartialEq)]
pub enum DataType {
  Id(u32),
  Text(String),
  Number(i32),
  Boolean(bool),
  Date(String),
  Array(Vec<DataType>),
  Document(Document),

}

pub type Document = HashMap<String, DataType>;

impl DataType {
  fn id(&self) -> u32 {
    match self {
      DataType::Id(id) => *id,
      _ => panic!("Not an ID"),
    }
  }
}


pub struct Collection {
  pub name: String,
  last_id: u32,
  pub(crate) data: Vec<Document>,
  //idTable: HashMap<u32, usize>
}



impl Collection {
  pub fn new(name: String) -> Self {
    Collection {
      name: name,
      last_id: 0,
      data: Vec::new(),
      //idTable: HashMap::new()
    }
  }

  pub fn add(&mut self, document: Document) {
    let mut document = document;
    if !document.contains_key("ID") {
      self.last_id += 1;
      document.insert("ID".to_string(), DataType::Id(self.last_id));
    }
    self.data.push(document);
  }

  pub fn rm(&mut self, id: u32) {
    //self.data.remove(index);
    let index = self.get_index(id);
    self.data.swap_remove(index);
  }

  pub fn count(&self) -> usize {
    self.data.len()
  }

  fn _get(&self, index: usize) -> Option<&Document> {
    self.data.get(index)
  }

  fn get_index(&self, id: u32) -> usize {
    let id = DataType::Id(id);
    self.data.iter().position(|x| x.get("ID").unwrap() == &id).unwrap()
  }

  pub fn getAll(&self) -> &Vec<Document> {
    &self.data
    
   }

  pub fn get(&self, id: u32) -> Option<&Document> {
    let id = DataType::Id(id);
    self.data.iter().find(|&x| x.get("ID").unwrap() == &id)
  }

  fn remove(&mut self, index: usize) -> Document {
    self.data.remove(index)
  }

}


//TEST
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_collection() {
    let mut collection = Collection::new("users".to_string());
    let mut document = Document::new();
    document.insert("id".to_string(), DataType::Id(1));
    document.insert("name".to_string(), DataType::Text("John".to_string()));
    document.insert("age".to_string(), DataType::Number(25));
    document.insert("isMarried".to_string(), DataType::Boolean(false));
    document.insert("birthDate".to_string(), DataType::Date("1995-01-01".to_string()));
    collection.add(document);
    assert!(collection._get(0).is_some());
  }
}

