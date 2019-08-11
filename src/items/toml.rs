use crate::items::traits::{Decorate, Builder};
use crate::items::util::{create_file};

static TOML: &'static str = r#"
[package]
name = "Replace Me"
version = "0.1.0"
authors = ["Replace Me"]
edition = "2018"
repository = "https://github.com/Replace Me.git"
readme = "README.md"
keywords = [ "Replace Me" ]
categories = [ "Replace Me" ]
license = "MIT/Apache-2.0"
exclude = [ "/.travis.yml" ]
description = "Replace Me"

[dependencies]
"#;

pub struct Toml {}
impl Toml {
    pub fn new() -> Self {
        Toml{}
    }
}

impl Decorate for Toml {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + "Cargo.toml";
        create_file(&name, TOML);
    }
}

///Builder
pub struct TomlBuilder {}
impl TomlBuilder {
    pub fn new() -> Self {
        TomlBuilder{}
    }
}

impl Builder for TomlBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( Toml::new() )
    }
}
