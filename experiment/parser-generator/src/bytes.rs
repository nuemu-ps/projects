use crate::traits::{Compare, CompareResult, Input, InputLength};
use crate::internal::{ErrorKind, Parser};

pub struct Tag<T> {
    tag: T,
}

impl<I, T> Parser<I> for Tag<T>
where
    I: Input + Compare<T>,
    T: InputLength + Clone,
{
    fn process(&mut self, i: I) -> Result<(I, I), (ErrorKind, I)> {
        let tag_len = self.tag.input_len();
        let t = self.tag.clone();

        match i.compare(t) {
            CompareResult::Ok => Ok((i.take_from(tag_len), i.take(tag_len))),
            CompareResult::Error => Err((ErrorKind::Tag, i)),
        }
    }
}

pub fn tag<T, I>(tag: T) -> impl Fn(I) -> Result<(I, I), (ErrorKind, I)>
where
  I: Input + Compare<T>,
  T: InputLength + Clone,
{
    move |i: I| {
        let mut parser = Tag {
            tag: tag.clone(),
        };

        parser.process(i)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample() {
        let (l, r) = tag("sam")("sample").unwrap();
        println!("l: {}, r: {}", l, r);
        panic!();
    }
}