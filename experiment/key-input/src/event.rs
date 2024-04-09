const TTY_BUFFER_SIZE: usize = 1_024;

use crate::event_source::get_or_insert_event_source;
use crate::parser::{parse, KeyCode};

use libc_wrapper::{Pollfd, POLLIN};

use std::io::Result;

pub enum Event {
    KeyPress(KeyCode),
    WindowResize,
}

pub fn read() -> Result<Event> {
    let event_source = get_or_insert_event_source()?;

    let tty_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().tty.fd,
        events: POLLIN,
        revents: 0,
    };

    let sig_winch_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().sig_winch.fd,
        events: POLLIN,
        revents: 0,
    };

    let mut fds = [tty_pollfd, sig_winch_pollfd];

    loop {
        if libc_wrapper::poll(&mut fds, None).is_err() {
            let err = std::io::Error::last_os_error();
            match err.kind() {
                std::io::ErrorKind::Interrupted => continue,
                _ => return Err(err),
            }
        }

        if fds[0].revents & POLLIN != 0 {
            let mut buffer = [0u8; TTY_BUFFER_SIZE];

            let result = event_source.as_ref().unwrap().tty.read(&mut buffer)?;
            let key_code = parse(&buffer[..(result as usize)])?;

            return Ok(Event::KeyPress(key_code));
        }

        if fds[1].revents & POLLIN != 0 {
            let mut buffer = [0u8; TTY_BUFFER_SIZE];

            let _ = event_source.as_ref().unwrap().sig_winch.read(&mut buffer)?;

            return Ok(Event::WindowResize);
        }
    }
}

pub fn poll(duration: Option<core::time::Duration>) -> std::io::Result<()> {
    let event_source = get_or_insert_event_source()?;

    let tty_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().tty.fd,
        events: POLLIN,
        revents: 0,
    };

    let sig_winch_pollfd = Pollfd {
        fd: event_source.as_ref().unwrap().sig_winch.fd,
        events: POLLIN,
        revents: 0,
    };

    let mut fds = [tty_pollfd, sig_winch_pollfd];

    if libc_wrapper::poll(&mut fds, duration).is_err() {
        let err = std::io::Error::last_os_error();
        match err.kind() {
            std::io::ErrorKind::Interrupted => Ok(()),
            _ => Err(err.into()),
        }
    } else {
        Ok(())
    }
}
