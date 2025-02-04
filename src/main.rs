mod memodb;
use memodb::{Collection, DataType, Document, MEMOdb};

fn main() {
    println!("Hello, world!");
    let mut db = MEMOdb::new();
    let abc = db.create_collection("abc");
    if abc.is_err() {
        return;
    }
    let mut abc = abc.unwrap();
    let mut doc = Document::new();
    doc.insert("Name".to_string(), DataType::Text("Alberto".to_string()));
    abc.add(doc);
}
