pub mod items;

use std::process::Command;

use crate::items::traits::{Decorate, Builder};
use crate::items::apache::ApacheBuilder;
use crate::items::example::ExamplesBuilder;
use crate::items::gitignore::GitIgnoreBuilder;
use crate::items::mit::MitBuilder;
use crate::items::readme::ReadmeBuilder;
use crate::items::toml::TomlBuilder;
use crate::items::yml::YmlBuilder;

const APACHE   : &'static str = "apache";
const EXAMPLE  : &'static str = "example";
const GITIGNORE: &'static str = "gitignore";
const MIT      : &'static str = "mit";
const README   : &'static str = "readme";
const TOML     : &'static str = "toml";
const YML      : &'static str = "yml";

static ITEMS: [&'static str; 7] = [APACHE, EXAMPLE, GITIGNORE, MIT, README, TOML, YML];

pub struct Items {
    pub components: Vec<Box<dyn Decorate>>,
}

impl Items {
    pub fn new(components: Vec<Box<dyn Decorate>>) -> Self {
        Items {
            components: components,
        }
    }
}

impl Decorate for Items {
    fn decorate(&mut self, path: &String) {
        for component in self.components.as_mut_slice() {
            component.decorate(&path);
        }
    }
}

pub struct ItemsBuilder{}
impl ItemsBuilder {
    pub fn new() -> Self {
        ItemsBuilder{}
    }
}
impl Builder for ItemsBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        let mut components = vec![];
         for &x in &ITEMS {
            match x {
                APACHE => {
                    let v = ApacheBuilder::new().build();
                    components.push(v);
                },

                EXAMPLE => {
                    let v = ExamplesBuilder::new().build();
                    components.push(v);
                },

                GITIGNORE => {
                    let v = GitIgnoreBuilder::new().build();
                    components.push(v);
                },

                MIT => {
                    let v = MitBuilder::new().build();
                    components.push(v);
                },

                README => {
                    let v = ReadmeBuilder::new().build();
                    components.push(v);
                },

                TOML => {
                    let v = TomlBuilder::new().build();
                    components.push(v);
                },

                YML => {
                    let v = YmlBuilder::new().build();
                    components.push(v);
                },

                _ => {
                    panic!("Error.");
                }
            }
        }

        Box::new(
            Items::new( components )
        )
    }
}


pub fn new(name: &str) {
    let _output = Command::new("cargo")
                         .arg("new")
                         .arg(&name)
                         .output()
                         .expect("Failed to execute command");
}

pub fn decorate(name: &str) {
    let path = "./".to_string() + name;

    ItemsBuilder::new().build().decorate(&path);
}
