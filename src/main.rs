extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

fn main() {
    load_risk_framework();
    // println!("Hello, world!");
}

fn load_risk_framework() {
    let mut file = File::open("src/framework.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    println!("{}", contents);
}
