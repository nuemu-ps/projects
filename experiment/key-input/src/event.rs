const TTY_BUFFER_SIZE: usize = 1_024;

use crate::parser::{parse, KeyCode};
use crate::event_source::{get_or_insert_event_source};

use std::os::fd::AsRawFd;
use libc_wrapper::{Pollfd, POLLIN};

pub fn read() -> std::io::Result<KeyCode> {
    let event_source = get_or_insert_event_source()?;

    let tty_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().tty,
        events: POLLIN,
        revents: 0,
    };

    let sig_winch_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().sig_winch.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };

    let mut fds = [tty_pollfd, sig_winch_pollfd];

    if libc_wrapper::poll(&mut fds, None).is_err() {
        let err = std::io::Error::last_os_error();
        match err.kind() {
            std::io::ErrorKind::Interrupted => {},
            _ => panic!("{}", err),
        }
    }
    let mut buffer = [0u8; TTY_BUFFER_SIZE];

    let result = event_source.as_ref().unwrap().read(&mut buffer)?;

    let key_code = parse(&buffer[..(result as usize)])?;

    Ok(key_code)
}

pub fn poll(duration: Option<core::time::Duration>) -> std::io::Result<()> {
    let event_source = get_or_insert_event_source()?;

    let tty_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().tty,
        events: POLLIN,
        revents: 0,
    };

    let sig_winch_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().sig_winch.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };

    let mut fds = [tty_pollfd, sig_winch_pollfd];

    if libc_wrapper::poll(&mut fds, duration).is_err() {
        let err = std::io::Error::last_os_error();
        match err.kind() {
            std::io::ErrorKind::Interrupted => Ok(()),
            _ => Err(err.into())
        }
    } else {
        Ok(())
    }
}
