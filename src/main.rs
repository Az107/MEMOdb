mod memodb;
use std::io::Write;
use std::path::{self, Path};
use std::{env, io};

use memodb::{Collection, DataType, Document, MEMOdb};

const DEFAULT_PATH: &str = "./MEMOdb/default.json";
const DEFAULT_COLLECTION_NAME: &str = "default";

fn process(default_collection: &mut Collection, action: &str, args: Vec<String>) {
    match action {
        "list" => {
            for (k, v) in default_collection.list() {
                println!("{} => {}", k, v.to_string())
            }
        }
        "set" => {
            if args.len() < 2 {
                println!("memodb set [key] [value]")
            }
            let key = args.get(0).unwrap().as_str();
            let value = args.get(1).unwrap().as_str();
            default_collection.add(key, DataType::Text(value.to_string()));
            println!("{} elements added", default_collection.count());
        }
        "echo" => {
            println!("{:?}", args);
        }
        _ => println!("Unknown command {}", action),
    }
}

fn main() {
    let mut db = MEMOdb::new();
    if !Path::new(DEFAULT_PATH).exists() {
        db.path = DEFAULT_PATH.to_string();
    } else {
        db = MEMOdb::load(DEFAULT_PATH).unwrap();
    }
    let default_collection = db.get_collection(DEFAULT_COLLECTION_NAME);
    let default_collection = if default_collection.is_none() {
        db.create_collection(DEFAULT_COLLECTION_NAME).unwrap()
    } else {
        default_collection.unwrap()
    };
    println!("MEMOdb {}", "MOCK");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);
            let command: Vec<String> = buffer.split_whitespace().map(|v| v.to_string()).collect();
            let action = command.get(0).unwrap().as_str();
            let args = if command.len() > 0 {
                command.clone()[1..].to_vec()
            } else {
                Vec::new()
            };
            if action == "exit" {
                break;
            }
            process(default_collection, action, args);
        }
    } else {
        let action = args.get(1).unwrap().as_str();
        let args = args.clone()[2..].to_vec();
        process(default_collection, action, args);
    }

    let _ = db.dump();
}
