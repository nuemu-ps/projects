#[derive(Debug)]
pub enum ErrorKind {
    Tag
}

pub trait Parser<Input> {
    fn process(&mut self, input: Input) -> Result<(Input, Input), (ErrorKind, Input)>;
}