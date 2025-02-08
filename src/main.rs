mod memodb;

use memodb::utils;
use std::fmt::format;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use memodb::{Collection, DataType, MEMOdb};

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
            let d = DataType::auto_load(value.to_string());
            if d.is_none() {
                println!("Unable to parse value");
            }
            let d = d.unwrap();
            collection.add(key, d);
            println!("{} element added", key);
        }
        "get" => {
            if args.len() < 1 {
                println!("memodb get [key]")
            }
            let key = args.get(0).unwrap().as_str();
            let value = collection.get(key);
            if value.is_none() {
                println!("{}: Not Found", key);
                return;
            }
            let value = value.unwrap().clone();
            if args.len() == 2 {
                let i = args.get(1).unwrap().parse::<usize>();
                if i.is_err() {
                    value;
                } else {
                    let i = i.unwrap();
                    println!("{}[{}] => {}", key, i, value.get(i).to_string());
                }
            } else {
                println!("{} => {}", key, value.to_string());
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
        let _ = db.create_collection(DEFAULT_COLLECTION_NAME);
    }
    let mut selected = String::new();
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
            } else if action == "select" {
                let new_selected = args[0].clone();
                if db.get_collection_list().contains(&(new_selected)) {
                    selected = new_selected.clone();
                } else {
                    println!("Collection don't exists");
                }
                continue;
            } else if selected.is_empty() && action == "list" {
                for c in db.get_collection_list() {
                    println!("-> {}", c);
                }
                continue;
            } else if action == "del_col" {
                if selected.is_empty() {
                    if args.len() != 0 {
                        selected = args[0].clone();
                    } else {
                        println!("No collection selected");
                        continue;
                    }
                }
                db.remove_collection(selected);
                selected = String::new();
            } else if action == "new" {
                if args.len() != 0 {
                    let _ = db.create_collection(&args[0]);
                } else {
                    println!("No collection name provided");
                }
                continue;
            }

            if selected == "" {
                println!("No collection selected");
                continue;
            }
            process(db.get_collection(selected.as_str()).unwrap(), action, args);
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
