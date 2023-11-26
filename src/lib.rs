use std::{cell::RefCell, mem};
use neon::prelude::*;
// import local module collection.rs
mod collection;
mod memodb;
use collection::{Collection, Document, DataType};
use memodb::MEMOdb;

type CapsuledMemodb = JsBox<RefCell<MEMOdb>>;


fn document_to_js_object<'a>(cx: &mut FunctionContext<'a>, document: &Document) -> JsResult<'a, JsObject> {
    let js_document: Handle<'_, JsObject> = cx.empty_object();
    for (k, v) in document {
        let key = cx.string(k.as_str());
        let value = cx.empty_object();
        js_document.set(cx, key, value)?;
    }
    Ok(js_document)
}

impl Collection {

    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let js_collection: Handle<'_, JsObject> = cx.empty_object();
        let name = cx.string(self.name.as_str());
        js_collection.set(cx, "name", name)?;
        let js_data: Handle<'_, JsArray> = cx.empty_array();
        for document in self.data.iter() {
            let js_document = document_to_js_object(cx, document)?;

        }
        Ok(js_collection)
    }   
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
    memodb.createCollection(name); 
    Ok(cx.undefined())
}

fn js_get_collection(mut cx: FunctionContext) -> JsResult<JsObject> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.getCollection(name).unwrap();
    let js_collection = collection.to_object(&mut cx)?;
    Ok(js_collection)
}

fn js_get_all_collections(mut cx: FunctionContext) -> JsResult<JsArray> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let collections = memodb.getAllCollections();
    //TODO: convert collections to js array
    let js_collections: Handle<'_, JsArray> = cx.empty_array();
    Ok(js_collections)
}

fn js_get_collection_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let collectionList = memodb.getCollectionList();
    let js_collectionList: Handle<'_, JsArray> = cx.empty_array();
    for (i, collection) in collectionList.iter().enumerate() {
        let js_collection = cx.string(collection);
        js_collectionList.set(&mut cx, i as u32, js_collection)?;
    }
    Ok(js_collectionList)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("new", js_new_memodb)?;
    cx.export_function("createCollection", js_create_collection)?;
    cx.export_function("getCollection", js_get_collection)?;
    cx.export_function("getAllCollections", js_get_all_collections)?;
    cx.export_function("getCollectionList", js_get_collection_list)?;

    Ok(())
}
