use neon::prelude::*;
// import local module collection.rs
mod collection;
mod memodb;
use collection::{Collection, Document, DataType};
use memodb::MEMOdb;

impl MEMOdb {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        //WORK in progress
        let js_object = cx.empty_object();
        return Ok(js_object);
    }
    
}



impl Collection {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let js_collection: Handle<'_, JsObject> = cx.empty_object();
        let name = cx.string(self.name.as_str());
        js_collection.set(cx, "name", name)?;
        let js_data: Handle<'_, JsArray> = cx.empty_array();
        for document in self.data.iter() {
            for (k, v) in document {
                let js_document = cx.empty_object();
                let key = cx.string(k.as_str());
                let value = cx.empty_object();
                js_document.set(cx, key, value)?;
            }

        }
        Ok(js_collection)
    }   
}
    





#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // cx.export_function("hello", hello)?;
    //export class MEMOdb

    Ok(())
}
