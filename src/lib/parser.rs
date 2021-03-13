use std::collections::HashMap;
use std::fs;
use pest::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[grammar = "systemd.pest"]
pub struct SystemdParser;

#[derive(Debug, Clone)]
pub enum SystemdValue {
    List(Vec<String>),
    Str(String),
}

fn pre_process_map(map: &mut HashMap<String, HashMap<String, SystemdValue>>) {
    for (_, value) in map.into_iter() {
	for (_, v) in value.into_iter() {
	    if let SystemdValue::List(vs) = v {
		if vs.len() == 0 {
		    let v_ = SystemdValue::Str(String::new());
		    *v = v_.clone();
		} else if vs.len() == 1 {
		    let v_ = SystemdValue::Str((vs[0]).clone());
		    *v = v_.clone();
		}
	    }
	}
    }
}

pub fn parse(name: &str) -> Result<HashMap<String, HashMap<String, SystemdValue>>> {

    let unparsed_file = fs::read_to_string(name)
	.with_context(|| format!("cannot read file {}", name))?;

    let file = SystemdParser::parse(Rule::file, &unparsed_file)
	.with_context(|| format!("unsuccessful parse"))?
	.next()
	.unwrap();
    
    let mut properties: HashMap<String, HashMap<String, SystemdValue>> =
	HashMap::new();

    let mut current_section_name = String::new();
    let mut current_key_name = String::new();

    for line in file.into_inner() {
	match line.as_rule() {
	    Rule::section => {
		let mut inner_rules = line.into_inner();
		// current_section_name = inner_rules.next().unwrap().as_str();
		current_section_name = inner_rules.next().unwrap().as_str().to_string();
	    },
	    Rule::property => {
		let mut inner_rules = line.into_inner();
		let section = properties.entry(current_section_name.clone()).or_default();
		
		let name = inner_rules.next().unwrap().as_str().to_string();
		let value = inner_rules.next().unwrap().as_str().to_string();

		if name == current_key_name {
		    let entry = section.entry(current_key_name.clone()).or_insert(SystemdValue::List(vec![]));
		    if let SystemdValue::List(ent) = entry {
			ent.push(value);
		    }
		} else {
		    let entry = section.entry(name.clone()).or_insert(SystemdValue::List(vec![]));
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
    
    Ok(properties)
}
