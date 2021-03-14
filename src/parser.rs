use std::collections::HashMap;
use std::fs;
use pest::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[grammar = "pest/systemd.pest"]
struct SystemdParser;

#[derive(Debug, Clone)]
/// Represents a variant type of Systemd unit file values.
pub enum SystemdValue {
    /// Wraps a String vector that contains multiple values for a duplicate key.
    List(Vec<String>),
    /// Wraps a String value of a respective key in the systemd unit file.
    Str(String),
}

/// Type alias for HashMap<String, HashMap<String, SystemdValue>>.
pub type SystemdUnit = HashMap<String, HashMap<String, SystemdValue>>;

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

/// Parses a given Systemd unit file at the given file path.
///
/// # Examples
///
/// ```
/// use systemd_parser::{parser, parser::SystemdValue};
///
/// if let Ok(u) = parser::parse("./unit_files/nginx.service") {
///     let unit = u.get(&"Unit".to_string()).unwrap();
///     let desc = unit.get(&"Description".to_string()).unwrap();
///     match desc {
///         SystemdValue::Str(v) => {
///             assert_eq!(*v, "Nginx".to_string());
///         },
///         _ => {
///             assert!(false);
///         },
///     }
/// } else {
///     assert!(false);
/// }
///
///```
pub fn parse(name: &str) -> Result<SystemdUnit> {

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
