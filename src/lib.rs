use std::cell::RefCell;
use neon::prelude::*;
// import local module collection.rs
mod collection;
mod memodb;
use collection::{Collection, Document, DataType};
use memodb::MEMOdb;

type CapsuledMemodb = JsBox<RefCell<MEMOdb>>;
type CapsuledCollection = JsBox<RefCell<Collection>>;




fn document_to_js_object<'a>(cx: &mut FunctionContext<'a>, document: &Document) -> JsResult<'a, JsObject> {
    let js_document: Handle<'_, JsObject> = cx.empty_object();
    for (k, v) in document {
        let key = cx.string(k.as_str());
        js_document.set(cx, key,  cx.null())?; 
    }
    Ok(js_document)
}

fn js_object_to_document<'a>(cx: &mut FunctionContext<'a>, js_object: &JsObject) -> Result<Document, & 'static str> {
    let mut document: Document = Document::new();
    let keys = js_object.get_own_property_names(cx).unwrap().to_vec(cx).unwrap();
    for key in keys {
        document.insert("".to_string(), collection::DataType::Text("".to_string())); //TODO get real value and cast to DataType
    }
    Ok(document)
}

impl Finalize for Collection {}

impl Collection {
    
    fn to_object<'a>(&mut self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let js_collection: Handle<'_, JsObject> = cx.empty_object();
        let name = cx.string(self.name.as_str());
        //let instance : CapsuledCollection = cx.boxed(RefCell::new(self));
        js_collection.set(cx, "name", name)?;
        js_collection.set(cx, "instance", name)?;
        
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
    memodb.create_collection(name); 
    Ok(cx.undefined())
}

fn js_get_collection(mut cx: FunctionContext) -> JsResult<JsObject> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let collection = memodb.get_collection(name).unwrap();
    let js_collection = collection.to_object(&mut cx)?;
    Ok(js_collection)
}

fn js_get_all_collections(mut cx: FunctionContext) -> JsResult<JsArray> {
    let _memodb  = cx.argument::<CapsuledMemodb>(0)?;
    let memodb = _memodb.borrow();
    let collections = memodb.get_all_collections();
    let js_collections: Handle<'_, JsArray> = cx.empty_array();
    for (i, collection) in collections.iter().enumerate() {
        let js_collection = collection.to_object(&mut cx)?;
        js_collections.set(&mut cx, i as u32, js_collection)?;
    }
    Ok(js_collections)
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
    let _collection  = cx.argument::<CapsuledCollection>(0)?;
    let mut collection = _collection.borrow_mut();
    let document = cx.argument::<JsObject>(1)?;
    let document = js_object_to_document(&mut cx, &document);
    collection.add(document.unwrap());
    Ok(cx.undefined())
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
