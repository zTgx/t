use std::fs::{OpenOptions};
use std::string::FromUtf8Error;
use std::io::prelude::*;
use std::process::Command;

pub fn create_file(name: &str, content: &str) {
    let mut file = OpenOptions::new().write(true).create(true).open(name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn get_git_name() -> Result<String, FromUtf8Error> {
    let output = Command::new("git")
                         .arg("config")
                         .arg("user.name")
                         .output()
                         .expect("Failed to execute command");

    let len = output.stdout.as_slice().len();
    if len <= 1 {
        panic!("No git user.name found.");
    }
    let len = len - 1;
    let output = output.stdout.as_slice().get(..len).unwrap().to_vec();

    String::from_utf8(output)
}
