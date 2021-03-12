extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::{fs, env::current_dir};
use pest::Parser;

#[derive(Parser)]
#[grammar = "systemd.pest"]
pub struct SystemdParser;

#[derive(Debug, Clone)]
pub enum SystemdValue<'a> {
    List(Vec<&'a str>),
    Str(&'a str),
}

fn main() {
    let mut dir = current_dir().unwrap();
    dir.push("src");
    dir.push("cardano-node.service");

    let name = dir.to_str().unwrap();
	
    let unparsed_file = fs::read_to_string(name)
	.expect("cannot read file");

    let file = SystemdParser::parse(Rule::file, &unparsed_file)
	.expect("unsuccessful parse")
	.next().unwrap();
    let mut properties: HashMap<&str, HashMap<&str, SystemdValue<'_>>> =
	HashMap::new();

    let mut current_section_name = "";
    let mut current_key_name = "";

    for line in file.into_inner() {
	match line.as_rule() {
	    Rule::section => {
		let mut inner_rules = line.into_inner();
		current_section_name = inner_rules.next().unwrap().as_str();
	    },
	    Rule::property => {
		let mut inner_rules = line.into_inner();
		let section = properties.entry(current_section_name).or_default();
		let name: &str = inner_rules.next().unwrap().as_str();
		let value: &str = inner_rules.next().unwrap().as_str();

		if name == current_key_name {
		    let entry = section.entry(&current_key_name).or_insert(SystemdValue::List(vec![]));
		    if let SystemdValue::List(ent) = entry {
			ent.push(value);
		    }
		} else {
		    let entry = section.entry(&name).or_insert(SystemdValue::List(vec![]));
		    if let SystemdValue::List(ent) = entry {
			ent.push(value);
		    }
		    current_key_name = name;
		}
	    },
	    Rule::EOI => (),
	    _ => unreachable!(),
	}
    }

    pre_process_map(&mut properties);

    println!("{:#?}", properties);
}

fn pre_process_map(map: &mut HashMap<&str, HashMap<&str, SystemdValue<'_>>>) {
    for (_, value) in map.into_iter() {
	for (_, v) in value.into_iter() {
	    if let SystemdValue::List(vs) = v {
		if vs.len() == 0 {
		    let v_ = SystemdValue::Str("");
		    *v = v_.clone();
		} else if vs.len() == 1 {
		    let v_ = SystemdValue::Str(vs[0]);
		    *v = v_.clone();
		}
	    }
	}
    }
    // map.clear();
}


    
