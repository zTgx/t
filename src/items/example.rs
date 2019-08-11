use crate::items::traits::{Decorate, Builder};
use std::fs;

pub struct Examples {}
impl Examples {
    pub fn new() -> Self {
        Examples {}
    }
}

impl Decorate for Examples {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + "examples";
        fs::create_dir(name).unwrap();
    }
}

///builder
pub struct ExamplesBuilder {}
impl ExamplesBuilder {
    pub fn new() -> Self {
        ExamplesBuilder{}
    }
}
impl Builder for ExamplesBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( Examples::new() )
    }
}
