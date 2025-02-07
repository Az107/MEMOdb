mod memodb;

use memodb::utils;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use memodb::{Collection, DataType, Document, MEMOdb};

const DEFAULT_PATH: &str = "default.mdb";
const DEFAULT_COLLECTION_NAME: &str = "default";

fn process(collection: &mut Collection, action: &str, args: Vec<String>) {
    match action {
        "list" => {
            for (k, v) in collection.list() {
                println!("{} => {}", k, v.to_string())
            }
        }
        "set" => {
            if args.len() < 2 {
                println!("memodb set [key] [value]")
            }
            let key = args.get(0).unwrap().as_str();
            let value = args.get(1).unwrap().as_str();
            collection.add(key, DataType::Text(value.to_string()));
            println!("{} elements added", collection.count());
        }
        "get" => {
            if args.len() < 1 {
                println!("memodb get [key]")
            }
            let key = args.get(0).unwrap().as_str();
            let value = collection.get(key);
            match value {
                Some(v) => println!("{}", v.to_string()),
                None => println!("{}: Not Found", key),
            };
        }
        "del" => {
            if args.len() < 1 {
                println!("memodb get [key]")
            }
            let key = args.get(0).unwrap().as_str();
            collection.rm(key);
            println!("{}: Removed", key);
        }
        "echo" => {
            println!("{:?}", args);
        }
        "name" => {
            println!("Collection {}", collection.name);
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
    println!("MEMOdb {}", db.version);
    if db.get_collection(DEFAULT_COLLECTION_NAME).is_none() {
        db.create_collection(DEFAULT_COLLECTION_NAME);
    }
    let mut selected = "";
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        loop {
            print!("{}> ", selected);
            let _ = io::stdout().flush();
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
            let command: Vec<String> = utils::smart_split(buffer);
            let action = command.get(0).unwrap().as_str();
            let args = if command.len() > 0 {
                command.clone()[1..].to_vec()
            } else {
                Vec::new()
            };

            if action == "exit" {
                break;
            }
            process(db.get_collection(selected).unwrap(), action, args);
        }
    } else {
        let action = args.get(1).unwrap().as_str();
        let args = args.clone()[2..].to_vec();
        process(
            db.get_collection(DEFAULT_COLLECTION_NAME).unwrap(),
            action,
            args,
        );
    }

    let _ = db.dump();
}
