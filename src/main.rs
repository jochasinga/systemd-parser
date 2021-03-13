extern crate pest;
#[macro_use]
extern crate pest_derive;

mod lib;
use lib::parser;
use std::env::current_dir;

fn main() {
    let mut dir = current_dir().unwrap();
    dir.push("src");
    dir.push("cardano-node.service");

    let name = dir.to_str().unwrap();

    if let Ok(p) = parser::parse(name) {
	println!("{:#?}", p);
    }
}



    
