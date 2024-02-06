use std::process::ExitCode;
use systemd_parser::parse;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("    cargo run --examples cli <systemd file>");

        return ExitCode::SUCCESS;
    }

    let systemd_unit_file_path = &args[1];
    match parse(systemd_unit_file_path) {
        Ok(parsed_content) => {
            println!("Parsed systemd unit file contents:");
            println!("{parsed_content:?}");

            ExitCode::SUCCESS
        }
        Err(err) => {
            println!("Failed to parse:");
            println!("{err:?}");

            ExitCode::FAILURE
        }
    }
}
