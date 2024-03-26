use std::io;

pub trait Command {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()>;
}
