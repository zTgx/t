use crate::items::traits::{Decorate, Builder};
use crate::items::util::{create_file};

static README: &'static str = r#"
README
"#;

pub struct Readme {}
impl Readme {
    pub fn new() -> Self {
        Readme{}
    }
}

impl Decorate for Readme {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + "README.md";
        create_file(&name, &README);
    }
}

///Builder
pub struct ReadmeBuilder {}
impl ReadmeBuilder {
    pub fn new() -> Self {
        ReadmeBuilder{}
    }
}

impl Builder for ReadmeBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( Readme::new() )
    }
}
