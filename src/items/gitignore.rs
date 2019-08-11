use crate::items::traits::{Decorate, Builder};
use crate::items::util::{create_file};

static IGNORE: &'static str = r#"
/target
**/*.rs.bk
Cargo.lock
"#;

pub struct GitIgnore {}
impl GitIgnore {
    pub fn new() -> Self {
        GitIgnore{}
    }
}

impl Decorate for GitIgnore {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + ".gitignore";
        create_file(&name, IGNORE);
    }
}

///Builder
pub struct GitIgnoreBuilder {}
impl GitIgnoreBuilder {
    pub fn new() -> Self {
        GitIgnoreBuilder{}
    }
}

impl Builder for GitIgnoreBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( GitIgnore::new() )
    }
}
