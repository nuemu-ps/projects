use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::io;

use crate::{csi, traits::Command};

pub fn enter_raw_mode() -> std::io::Result<()> {
    let fd = get_tty_file_descriptor()?;

    let mut termios = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    }; // TODO: learn termios

    unsafe { libc::tcgetattr(fd, &mut termios) };
    unsafe { libc::cfmakeraw(&mut termios) };
    unsafe { libc::tcsetattr(fd, 0, &mut termios) };

    Ok(())
}

pub fn get_tty_file_descriptor() -> std::io::Result<i32> {
    let fd = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        libc::STDIN_FILENO
    } else {
        panic!();
    };

    Ok(fd)
}


#[derive(Debug)]
pub struct WindowSize {
    pub rows: u16,
    pub cols: u16,
}

impl From<winsize> for WindowSize {
    fn from(size: winsize) -> WindowSize {
        WindowSize {
            cols: size.ws_col,
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
