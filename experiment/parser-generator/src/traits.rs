pub trait Input: Clone {
    fn take(&self, index: usize) -> Self;
    fn take_from(&self, index: usize) -> Self;
}

impl<'a> Input for &'a str {
    fn take(&self, index: usize) -> Self {
        &self[..index]
    }

    fn take_from(&self, index: usize) -> Self {
        &self[index..]
    }
}

pub enum CompareResult {
    Ok,
    Error,
}

pub trait Compare<T> {
    fn compare(&self, t: T) -> CompareResult;
}

impl<'a, 'b> Compare<&'b [u8]> for &'a [u8] {
    fn compare(&self, t: &'b [u8]) -> CompareResult {
        let pos = self.iter().zip(t.iter()).position(|(a, b)| a != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                CompareResult::Ok
            }
        }
    }
}

impl<'a, 'b> Compare<&'b str> for &'a str {
    fn compare(&self, t: &'b str) -> CompareResult {
        self.as_bytes().compare(t.as_bytes())
    }
}

pub trait InputLength {
    fn input_len(&self) -> usize;
}

impl<'a, T> InputLength for &'a [T] {
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'a> InputLength for &'a str {
    fn input_len(&self) -> usize {
        self.len()
    }
}
