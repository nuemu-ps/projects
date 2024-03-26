use core::fmt::Display;
use std::io;

use crate::traits::Command;

pub struct Print<T>(pub T);

impl<T: Display> Command for Print<T> {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, "{}", self.0)?;
        Ok(())
    }
}
