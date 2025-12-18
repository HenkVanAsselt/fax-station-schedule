// use anyhow::Result;
// use assert_cmd::Command;
// use predicates::prelude::*;
// use pretty_assertions::assert_eq;
// use rand::{Rng, distributions::Alphanumeric};
// use std::fs;
// use assert_cmd::assert;

use schedule::load_transmission_schedule;
use std::env;
use std::path::Path;

// --------------------------------------------------
#[test]
fn works() {
    assert!(true);
}

// --------------------------------------------------
#[test]
fn load_nonexisting_csv() {
    assert!(load_transmission_schedule("tests/test_files/non_existing.csv").is_err());
}

fn show_dir() {
    let dir = env::current_dir();
    println!(
        "The current directory is {}",
        dir.expect("REASON").display()
    );
}

// --------------------------------------------------
#[test]
fn load_test_csv() {
    show_dir();
    let filename = "./tests//test_files/test.csv";
    assert!(Path::new(filename).is_file());
    assert!(load_transmission_schedule(filename).is_ok());
}
