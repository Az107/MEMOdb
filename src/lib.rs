use std::{any::{Any, TypeId}, cell::{RefCell, RefMut}, collections::HashMap, env, hash::Hash, mem, ptr::null, rc::Rc};
// import local module collection.rs

mod memodb;

use memodb::collection::{Collection, Document};
use memodb::MEMOdb;
use memodb::dataType::DataType;

use napi::{bindgen_prelude::{Null, Object, ToNapiValue}, Env, JsObject};
use napi_derive::napi;
use napi::bindgen_prelude::FromNapiValue;

#[macro_use]
extern crate napi_derive;
//create a memodbjs struct that herit from MEMOdb
impl FromNapiValue for DataType {
    unsafe fn from_napi_value(env: napi::sys::napi_env, napi_val: napi::sys::napi_value) -> napi::Result<Self> {
       //get the type of the napi value
        let mut type_id: i32 =  napi::sys::napi_valuetype::default();
        napi::sys::napi_typeof(env, napi_val, &mut type_id);
        match type_id {
            napi::sys::ValueType::napi_string => {
                let mut result: String = String::from_napi_value(env, napi_val)?;
                //error for test
                Ok(Self::Text(result))
            }
            napi::sys::ValueType::napi_number => {
                let mut result: i32 = f64::from_napi_value(env, napi_val)?.trunc() as i32;
                Ok(Self::Number(result))
            }
            _ => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    "Error converting napi value to DataType",
                ))
            }

        }
       
    }
}

impl ToNapiValue for DataType {
    unsafe fn to_napi_value(env: napi::sys::napi_env, val: Self) -> napi::Result<napi::sys::napi_value>{
        let mut result: napi::sys::napi_value = 0 as napi::sys::napi_value;
        match val {
            DataType::Id(id) => {
                napi::sys::napi_create_int32(env, id.try_into().unwrap(), &mut result);
                Ok(result)
            }
            DataType::Text(text) => {
                let text_ptr = text.as_ptr() as *const i8;
                napi::sys::napi_create_string_utf8(env, text_ptr, text.len(), &mut result);
                Ok(result)
            }
            DataType::Number(number) => {
                napi::sys::napi_create_int32(env, number, &mut result);
                Ok(result)
            }
            DataType::Boolean(boolean) => {
                napi::sys::napi_get_boolean(env, boolean, &mut result);
                Ok(result)
            }
            DataType::Date(date) => {
                let text_ptr = date.as_ptr() as *const i8;
                napi::sys::napi_create_string_utf8(env, text_ptr,date.len(), &mut result);
                Ok(result)
            }
            DataType::Array(array) => {
                napi::sys::napi_create_array_with_length(env, array.len(), &mut result);
                Ok(result)
            }
            // DataType::Document(document) => {
            //     let result: napi::sys::napi_value = "[object Placeholder]".as_ptr() as napi::sys::napi_value;
            //     Ok(result)
            // }
            _ => {
                Err(napi::Error::new(
                    napi::Status::GenericFailure,
                    "Error converting DataType to napi value",
                ))
            }
        }
    }
}

#[napi(js_name = "MEMOdb")]
pub struct MEMOdbJS {
    db: Rc<RefCell<MEMOdb>>,
}

#[napi(js_name = "Collection")]
pub struct CollectionJS {
    pub name: String,
    db: Rc<RefCell<MEMOdb>>,
}

#[napi]
impl CollectionJS {
    pub fn new(name: String, db: Rc<RefCell<MEMOdb>>) -> Self {
        CollectionJS {
            db,
            name,
        }
    }

    #[napi]
    pub fn add(&self, document: Document) -> u32 {
        let mut binding = self.db.borrow_mut();
        let collection = binding.get_collection(self.name.clone()).unwrap();
        collection.add(document)
    }

    #[napi]
    pub fn get(&self, id: u32) -> Option<Document> {
        let mut binding = self.db.borrow_mut();
        let collection = binding.get_collection(self.name.clone()).unwrap();
        let document: Option<&Document> = collection.get(id);
        match document {
            Some(document) => Some(document.clone()),
            None => None,
            
        }
    }    

    #[napi]
    pub fn remove(&self, id: u32) {
        let mut binding = self.db.borrow_mut();
        let collection = binding.get_collection(self.name.clone()).unwrap();
        collection.rm(id);
    }
}

#[napi]
impl MEMOdbJS {
    #[napi(constructor)]
    pub fn new() -> Self {
        MEMOdbJS {
            db: RefCell::new(MEMOdb::new()).into(),
        }
    }

    #[napi]
    pub fn create_collection(&self, name: String) {
        self.db.borrow_mut().create_collection(name);
    }

    //list all collections
    #[napi]
    pub fn get_collection_list(&self) -> Vec<String> {
        self.db.borrow().get_collection_list()
    }

    //get a collection
    #[napi]
    pub fn get_collection(&self, name: String) -> Option<CollectionJS> {
        let mut binding = self.db.borrow_mut();
        let collection = binding.get_collection(name.clone());
        match collection {
            Some(collection) => Some(CollectionJS::new(collection.name.clone(), Rc::clone(&self.db))),
            None => None,
        }
    }


   
}