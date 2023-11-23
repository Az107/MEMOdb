use std::collections::HashMap;


#[derive(PartialEq)]
pub enum DataType {
  Id(u32),
  Text(String),
  Number(i32),
  Boolean(bool),
  Date(String),
  Array(Vec<DataType>),

}

pub type Document = HashMap<String, DataType>;

pub struct Collection {
  name: String,
  data: Vec<Document>,
}



impl Collection {
  fn new(name: String) -> Self {
    Collection {
      name: name,
      data: Vec::new()
    }
  }

  fn add(&mut self, document: Document) {
    self.data.push(document);
  }

  fn get(&self, index: usize) -> Option<&Document> {
    self.data.get(index)
  }

  fn getAll(&self) -> &Vec<Document> {
    &self.data
  }

  fn getById(&self, id: u32) -> Option<&Document> {
    let Id = DataType::Id(id);
    self.data.iter().find(|&x| x.get("id").unwrap() == &Id)
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
    assert!(collection.get(0).is_some());
  }
}

