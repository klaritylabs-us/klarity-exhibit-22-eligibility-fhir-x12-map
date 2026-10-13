use std::{fs, path::PathBuf};
use eligibility_map::validate_eligibility_file;

fn ex(n: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples");
    p.push(n);
    fs::read_to_string(p).unwrap()
}

#[test]
fn valid_json_fixture() {
    let s = ex("valid-eligibility-request.json");
    let r = validate_eligibility_file(&s);
    assert!(r.is_valid());
}

#[test]
fn invalid_json_fixture() {
    let s = ex("invalid-eligibility-request.json");
    let r = validate_eligibility_file(&s);
    assert!(!r.is_valid());
}

#[test]
fn valid_x12_sketch_fixture() {
    let s = ex("valid-271-sketch.txt");
    let r = validate_eligibility_file(&s);
    assert!(r.is_valid());
}
