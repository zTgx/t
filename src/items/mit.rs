use crate::items::traits::{Decorate, Builder};
use crate::items::util::{create_file, get_git_name};

static MIT_LICENSE: &'static str = r#"
    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
"#;

pub struct Mit {}
impl Mit {
    pub fn new() -> Self {
        Mit{}
    }
}

impl Decorate for Mit {
    fn decorate(&mut self, path: &String) {
        let name = path.to_owned() + "/" + "LICENSE-MIT";

        let git_user_name = get_git_name().unwrap();
        let content_header = "      Copyright (c) 2019 ".to_owned() + &git_user_name + "\n";
        let content = content_header + MIT_LICENSE;
        create_file(&name, &content);
    }
}

///Builder
pub struct MitBuilder {}
impl MitBuilder {
    pub fn new() -> Self {
        MitBuilder{}
    }
}

impl Builder for MitBuilder {
    type Output = Box<dyn Decorate>;
    fn build(&mut self) -> Box<dyn Decorate> {
        Box::new( Mit::new() )
    }
}
