use crate::items::traits::{Decorate, Builder};
use crate::items::util::{create_file};

static YML: &'static str = r#"
language: rust
sudo: false

rust:
  - stable

script:
  - cargo build --verbose --all

"#;

pub struct Yml {}
impl Yml {
    pub fn new() -> Self {
        Yml{}
    }
}

impl Decorate for Yml {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + ".travis.yml";
        create_file(&name, &YML);
    }
}

///Builder
pub struct YmlBuilder {}
impl YmlBuilder {
    pub fn new() -> Self {
        YmlBuilder{}
    }
}

impl Builder for YmlBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( Yml::new() )
    }
}
