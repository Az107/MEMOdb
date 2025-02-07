// Written by Alberto Ruiz 2024-03-08
// The data type module will provide the data types for the MEMOdb
// this will be store several types of data, like text, numbers, dates, arrays and documents
//
// The data type will be used to store the data in the documents
use super::collection::{Document, DocumentJson};
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub enum DataType {
    Id(Uuid),
    Text(String),
    Number(i32),
    Boolean(bool),
    Array(Vec<DataType>),
    Document(Document),
}

impl DataType {
    pub fn get_type(&self) -> &str {
        match self {
            DataType::Id(_) => "id",
            DataType::Text(_) => "text",
            DataType::Number(_) => "number",
            DataType::Boolean(_) => "boolean",
            DataType::Array(_) => "array",
            DataType::Document(_) => "document",
        }
    }

    //add into
    pub fn to_id(&self) -> Uuid {
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

    pub fn auto_load(raw: String) -> Option<Self> {
        let t = if Uuid::parse_str(raw.as_str()).is_ok() {
            1
        } else if raw.parse::<i32>().is_ok() {
            3
        } else if raw.to_lowercase().as_str() == "true" || raw.to_lowercase().as_str() == "true" {
            4
        } else {
            2
        };

        Self::load(t, raw)
    }

    pub fn load(t: u16, raw: String) -> Option<Self> {
        match t {
            1 => {
                let id = Uuid::parse_str(raw.as_str());
                if id.is_err() {
                    return None;
                }
                Some(DataType::Id(id.unwrap()))
            }
            2 => Some(DataType::Text(raw)),
            3 => {
                let n = raw.parse::<i32>();
                if n.is_err() {
                    return None;
                }
                Some(DataType::Number(n.unwrap()))
            }
            4 => match raw.to_lowercase().as_str() {
                "true" => Some(DataType::Boolean(true)),
                "false" => Some(DataType::Boolean(false)),
                _ => None,
            },
            _ => None,
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

impl From<Uuid> for DataType {
    fn from(value: Uuid) -> Self {
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

//impl clone
impl Clone for DataType {
    fn clone(&self) -> Self {
        match self {
            DataType::Id(id) => DataType::Id(*id),
            DataType::Text(text) => DataType::Text(text.clone()),
            DataType::Number(number) => DataType::Number(*number),
            DataType::Boolean(boolean) => DataType::Boolean(*boolean),
            DataType::Array(array) => DataType::Array(array.clone()),
            DataType::Document(document) => DataType::Document(document.clone()),
        }
    }
}
