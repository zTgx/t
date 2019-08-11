pub trait Decorate {
    fn decorate(&mut self, path: &String);
}

pub trait Builder {
    type Output;
    fn build(&mut self) -> Self::Output;
}
