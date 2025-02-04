mod memodb;
use memodb::MEMOdb;

fn main() {
    println!("Hello, world!");
    MEMOdb::new().create_collection("abc");
}
