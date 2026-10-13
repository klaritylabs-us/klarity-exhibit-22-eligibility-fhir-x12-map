use std::{env, fs, process::ExitCode};
use eligibility_map::validate_eligibility_file;

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: validate-eligibility <file>");
            return ExitCode::from(2);
        }
    };
    let text = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return ExitCode::from(2);
        }
    };
    let report = validate_eligibility_file(&text);
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap_or_else(|_| "[]".into()));
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
