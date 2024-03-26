use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io;

use crate::{csi, traits::Command};

#[derive(Debug)]
pub struct WindowSize {
    pub rows: u16,
    pub columns: u16,
}

impl From<winsize> for WindowSize {
    fn from(size: winsize) -> WindowSize {
        WindowSize {
            columns: size.ws_col,
            rows: size.ws_row,
        }
    }
}

pub fn window_size() -> io::Result<WindowSize> {
    let w = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe {
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == 0 {
            let window_size: WindowSize = w.into();
            return Ok(window_size);
        } else {
            Err(std::io::Error::last_os_error().into())
        }
    }
}

pub struct EnterAlternateScreen;

impl Command for EnterAlternateScreen {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, csi!("?1049h"))
    }
}

pub struct LeaveAlternateScreen;

impl Command for LeaveAlternateScreen {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, csi!("?1049l"))
    }
}

pub enum ClearType {
    All,
}

pub struct Clear(pub ClearType);

impl Command for Clear {
    fn write_ansi(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self.0 {
            ClearType::All => write!(writer, csi!("2J")),
        }
    }
}
