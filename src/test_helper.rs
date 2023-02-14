#![cfg(test)]

use std::fs;

pub fn read_test_file(filename: &str) -> String {
    fs::read_to_string(&format!("{}/{}", TEST_FILES_DIR, filename)).unwrap()
}

const TEST_FILES_DIR: &str = r#"/home/lauglam/Documents/RustCode/ehviewer-x-tests"#;
