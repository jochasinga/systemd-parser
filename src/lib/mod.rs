pub mod parser;

#[cfg(test)]
mod test {

    use super::*;
    use parser::{parse, SystemdValue};
    use std::env::current_dir;

    #[test]
    fn test_parse_systemd_str() {
	let mut dir = current_dir().unwrap();
	dir.push("unit_files");
	dir.push("cardano-node.service");
	let filepath = dir.to_str().unwrap();
	if let Ok(p) = parse(filepath) {
	    let unit = p.get(&String::from("Unit")).unwrap();
	    let mut test_pairs = vec![
		(
		    unit.get(&String::from("Description")).unwrap(),
		    String::from("Cardano Node"),
		),
		(
		    unit.get(&String::from("After")).unwrap(),
		    String::from("network-online.target"),
		),
		( 
		    unit.get(&String::from("Wants")).unwrap(),
		    String::from("network-online.target"),
		),
	    ];
	    for tp in test_pairs.into_iter() {
		match tp {
		    (SystemdValue::Str(s), expect) => {
			assert_eq!(*s, *expect);
		    },
		    _ => assert!(false),
		}
	    }

	    let srv = p.get(&String::from("Service")).expect("Holy shit");
	    test_pairs = vec![
		(
		    srv.get(&String::from("Type")).unwrap(),
		    String::from("simple"),
		),
		(
		    srv.get(&String::from("ExecStart")).unwrap(),
		    String::from("/usr/local/sbin/relay-init.sh"),
		),
		( 
		    srv.get(&String::from("Restart")).unwrap(),
		    String::from("on-failure"),
		),
		( 
		    srv.get(&String::from("RestartSec")).unwrap(),
		    String::from("3"),
		),
		( 
		    srv.get(&String::from("KillMode")).unwrap(),
		    String::from("process"),
		),
	    ];	    
	    for tp in test_pairs.into_iter() {
		match tp {
		    (SystemdValue::Str(s), expect) => {
			assert_eq!(*s, *expect);
		    },
		    _ => assert!(false),
		}
	    }

	    let install = p.get(&String::from("Install")).unwrap();
	    test_pairs = vec![
		(
		    install.get(&String::from("WantedBy")).unwrap(),
		    String::from("multi-user.target"),
		)
	    ];
	    for tp in test_pairs.into_iter() {
		match tp {
		    (SystemdValue::Str(s), expect) => {
			assert_eq!(*s, *expect);
		    },
		    _ => assert!(false),
		}
	    }
	} else {
	    assert!(false);
	}
    }
}
