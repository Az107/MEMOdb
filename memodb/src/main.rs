mod command;
mod memodb;
mod server;

use memodb::utils;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use command::Command;
use memodb::{Collection, MEMOdb};

const DEFAULT_PATH: &str = "default.mdb";
const DEFAULT_COLLECTION_NAME: &str = "default";

fn process(collection: &mut Collection, command: &str) -> Result<String, &'static str> {
    let r = collection.run(command)?;
    return Ok(r.to_string());
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
            let command: Vec<String> = utils::smart_split(buffer.clone());
            let action = command.get(0).unwrap();
            let args = if command.len() > 0 {
                command.clone()[1..].to_vec()
            } else {
                Vec::new()
            };

            if action == "exit" {
                break;
            } else if action == "select" {
                if db.get_collection_list().contains(&args[0]) {
                    selected = args[0].clone()
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
            let r = process(db.get_collection(selected.as_str()).unwrap(), &buffer);
            println!("{}", r.unwrap_or("0_o".to_string()));
        }
    } else {
        let command = args.clone()[1..].to_vec().join(" ");
        let r = process(
            db.get_collection(DEFAULT_COLLECTION_NAME).unwrap(),
            &command,
        );
        println!("{}", r.unwrap_or("0_o".to_string()));
    }

    let _ = db.dump();
}
