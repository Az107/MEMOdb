use crate::collection::Document;

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

impl DataType {
  //add into 
  pub fn to_id(&self) -> u32 {
    match self {
      DataType::Id(id) => *id,
      _ => panic!("Not an ID"),
    }
  }
  pub fn to_text(&self) -> &String {
    match self {
      DataType::Text(text) => text,
      _ => panic!("Not a Text"),
    }
  }
  pub fn to_number(&self) -> i32 {
    match self {
      DataType::Number(number) => *number,
      _ => panic!("Not a Number"),
    }
  }
  pub fn to_boolean(&self) -> bool {
    match self {
      DataType::Boolean(boolean) => *boolean,
      _ => panic!("Not a Boolean"),
    }
  }
  pub fn to_date(&self) -> &String {
    match self {
      DataType::Date(date) => date,
      _ => panic!("Not a Date"),
    }
  }
  pub fn to_array(&self) -> &Vec<DataType> {
    match self {
      DataType::Array(array) => array,
      _ => panic!("Not an Array"),
    }
  }
  pub fn to_document(&self) -> &Document {
    match self {
      DataType::Document(document) => document,
      _ => panic!("Not a Document"),
    }
  }
  
}

impl ToString for DataType {
  fn to_string(&self) -> String {
    match self {
      DataType::Id(id) => id.to_string(),
      DataType::Text(text) => text.to_string(),
      DataType::Number(number) => number.to_string(),
      DataType::Boolean(boolean) => boolean.to_string(),
      DataType::Date(date) => date.to_string(),
      DataType::Array(array) => {
        let mut result = String::new();
        for value in array {
          result.push_str(&value.to_string());
          result.push_str(", ");
        }
        result
      }
      DataType::Document(document) => {
        let mut result = String::new();
        for (key, value) in document {
          result.push_str(&key);
          result.push_str(": ");
          result.push_str(&value.to_string());
          result.push_str(", ");
        }
        result
      }
    }
  }
}


impl From<u32> for DataType {
  fn from(value: u32) -> Self {
    DataType::Id(value)
  }
}

impl From<String> for DataType {
  fn from(value: String) -> Self {
    DataType::Text(value)
  }
}

impl From<&str> for DataType {
  fn from(value: &str) -> Self {
    DataType::Text(value.to_string())
  }
}

impl From<i32> for DataType {
  fn from(value: i32) -> Self {
    DataType::Number(value)
  }
}

impl From<bool> for DataType {
  fn from(value: bool) -> Self {
    DataType::Boolean(value)
  }
}

impl From<Vec<DataType>> for DataType {
  fn from(value: Vec<DataType>) -> Self {
    DataType::Array(value)
  }
}

impl From<Document> for DataType {
  fn from(value: Document) -> Self {
    DataType::Document(value)
  }
}