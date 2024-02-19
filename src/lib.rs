use std::{cell::RefCell, mem, ptr::null};
use neon::{prelude::*, types::buffer::Ref};
// import local module collection.rs
mod collection;
mod memodb;
use collection::{Collection, Document, DataType};
use memodb::MEMOdb;

type CapsuledMemodb = JsBox<RefCell<MEMOdb>>;


pub struct FinalizableCollection(RefCell<Collection>);

impl Finalize for FinalizableCollection {}

fn document_to_js_object<'a>(cx: &mut FunctionContext<'a>, document: &Document) -> JsResult<'a, JsObject> {
    let js_document: Handle<'_, JsObject> = cx.empty_object();
    for (k, v) in document {
        let key = cx.string(k.as_str());
        match v {
            DataType::Number(n) => {
                let value = cx.number(*n);
                js_document.set(cx, key, value)?;
            },
            DataType::Id(n) => {
                let value = cx.number(*n);
                js_document.set(cx, key, value)?;
            },
            DataType::Text(n) => {
                let value = cx.string(n);
                js_document.set(cx, key, value)?;
            },
            DataType::Boolean(n) => {
                let value = cx.boolean(*n);
                js_document.set(cx, key, value)?;
            },
            DataType::Date(n) => {
                let value = cx.string(n);
                js_document.set(cx, key, value)?;
            },
            DataType::Array(n) => {
                let value = cx.empty_array();
                js_document.set(cx, key, value)?;
            },
            DataType::Document(_) => todo!()
        };
    }
    Ok(js_document)
}

fn js_object_to_document<'a>(cx: &mut FunctionContext<'a>, js_object: &JsObject) -> Result<Document, & 'static str> {
    let mut document: Document = Document::new();
    //iter over js_object properties
    let keys = js_object.get_own_property_names(cx).unwrap().to_vec(cx).unwrap();
    for key in keys {
        let key_name = key.downcast::<JsString, _>(cx).unwrap().value(cx);
        let value = js_object.get_value(cx, key).unwrap();
        //check type of value
        if value.is_a::<JsNumber, _>(cx) {
            let value = value.downcast::<JsNumber, _>(cx).unwrap().value(cx);
            document.insert(key_name, DataType::Number(value as i32));
        } else if value.is_a::<JsString, _>(cx) {
            let value = value.downcast::<JsString, _>(cx).unwrap().value(cx);
            document.insert(key_name, DataType::Text(value));
        } else if value.is_a::<JsBoolean, _>(cx) {
            let value = value.downcast::<JsBoolean, _>(cx).unwrap().value(cx);
            document.insert(key_name, DataType::Boolean(value));
        } else if value.is_a::<JsArray, _>(cx) {
            let value = value.downcast::<JsArray, _>(cx).unwrap().to_vec(cx).unwrap();
            let mut value2: Vec<DataType> = Vec::new();
            for element in value {
                let element = element.downcast::<JsObject, _>(cx).unwrap();
                let element = js_object_to_document(cx, &element);
                if element.is_err() {
                    continue;
                }
                value2.push(DataType::Document(element.unwrap()));
            }
            document.insert(key_name, DataType::Array(value2));
        } else {
            return Err("Unknown type");
        }

    }

    Ok(document)
}



impl Finalize for MEMOdb {}

//MEMOdb wrapper
fn js_new_memodb(mut cx: FunctionContext) -> JsResult<CapsuledMemodb> {
    let memodb = MEMOdb::new();
    Ok(cx.boxed(RefCell::new(memodb)))
}

fn js_create_collection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    memodb.create_collection(name); 
    Ok(cx.undefined())
}

fn js_get_collection_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let collection_list = memodb.get_collection_list();
    let js_collection_list: Handle<'_, JsArray> = cx.empty_array();
    for (i, collection) in collection_list.iter().enumerate() {
        let js_collection = cx.string(collection);
        js_collection_list.set(&mut cx, i as u32, js_collection)?;
    }
    Ok(js_collection_list)
}

// Collection

fn js_collection_add(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let _collection  = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(_collection);
    if collection.is_none() {
        return Err(cx.throw_error("Collection not found").unwrap());
    }
    let collection = collection.unwrap();
    let document = cx.argument::<JsObject>(2)?;
    let document = js_object_to_document(&mut cx, &document);
    collection.add(document.unwrap());
    Ok(cx.undefined())
}

fn js_collection_get(mut cx: FunctionContext) -> JsResult<JsObject> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let _collection  = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(_collection);
    if collection.is_none() {
        return Err(cx.throw_error("Collection not found").unwrap());
    }
    let collection = collection.unwrap();
    let index = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let document = collection.get(index as u32);
    if document.is_none() {
        return Err(cx.throw_error("Document not found").unwrap());
    }
    let document = document.unwrap();
    let js_document = document_to_js_object(&mut cx, &document);
    Ok(js_document.unwrap())
}

fn js_collection_get_all(mut cx: FunctionContext) -> JsResult<JsArray> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let _collection  = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(_collection);
    if collection.is_none() {
        return Err(cx.throw_error("Collection not found").unwrap());
    }
    let collection = collection.unwrap();
    let documents = collection.getAll();
    let js_documents: Handle<'_, JsArray> = cx.empty_array();
    for (i, document) in documents.iter().enumerate() {
        let js_document = document_to_js_object(&mut cx, &document);
        js_documents.set(&mut cx, i as u32, js_document.unwrap())?;
    }
    Ok(js_documents)
}

fn js_collection_rm(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let _collection  = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(_collection);
    if collection.is_none() {
        return Err(cx.throw_error("Collection not found").unwrap());
    }
    let collection = collection.unwrap();
    let index = cx.argument::<JsNumber>(2)?.value(&mut cx);
    collection.remove(index as usize);
    Ok(cx.undefined())
}

fn js_collection_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let mut memodb = _memodb.borrow_mut();
    let _collection  = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(_collection);
    if collection.is_none() {
        return Err(cx.throw_error("Collection not found").unwrap());
    }
    let collection = collection.unwrap();
    let count = collection.count();
    Ok(cx.number(count as f64))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("new", js_new_memodb)?;
    cx.export_function("createCollection", js_create_collection)?;
    cx.export_function("getCollectionList", js_get_collection_list)?;
    cx.export_function("collectionAdd", js_collection_add)?;
    cx.export_function("collectionGet", js_collection_get)?;
    cx.export_function("collectionGetAll", js_collection_get_all)?;
    cx.export_function("collectionRm", js_collection_rm)?;
    cx.export_function("collectionCount", js_collection_count)?;

    Ok(())
}


