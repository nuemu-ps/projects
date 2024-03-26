use crate::csi;
use crate::traits::Command;
use std::io;

#[derive(Debug)]
pub struct MoveTo(pub u16, pub u16);

impl Command for MoveTo {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, csi!("{};{}H"), self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug)]
pub struct Show;

impl Command for Show {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, csi!("?25h"))
    }
}

#[derive(Debug)]
pub struct Hide;

impl Command for Hide {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, csi!("?25l"))
    }
}
