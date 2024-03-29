const TTY_BUFFER_SIZE: usize = 1_024;

use crate::parser::{parse, KeyCode};
use crate::terminal::get_tty_file_descriptor;

#[derive(Copy, Clone)]
struct EventSource {
    tty: i32,
}

use std::sync::{Mutex, MutexGuard};

static EVENT_SOURCE: Mutex<Option<EventSource>> = Mutex::new(None);

impl EventSource {
    fn new() -> std::io::Result<Self> {
        Ok(EventSource {
            tty: get_tty_file_descriptor()?,
        })
    }

    pub fn read(&self, buffer: &mut [u8]) -> std::io::Result<isize> {
        let result = unsafe {
            libc::read(
                self.tty,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len() as libc::size_t,
            )
        };

        if result < 0 {
            panic!("{}", std::io::Error::last_os_error());
        } else {
            Ok(result)
        }
    }
}

fn get_or_insert_event_source() -> std::io::Result<MutexGuard<'static, Option<EventSource>>> {
    if let Ok(mut optional_event_source) = EVENT_SOURCE.lock() {
        if optional_event_source.is_none() {
            *optional_event_source = Some(EventSource::new()?);
        }
        return Ok(optional_event_source);
    }
    panic!("{}", std::io::Error::last_os_error())
}

pub fn read() -> std::io::Result<KeyCode> {
    let event_source = get_or_insert_event_source()?;
    let mut buffer = [0u8; TTY_BUFFER_SIZE];

    let result = (*event_source).unwrap().read(&mut buffer)?;

    let key_code = parse(&buffer[..(result as usize)])?;

    Ok(key_code)
}

pub fn poll(duration: core::time::Duration) -> std::io::Result<()> {
    let event_source = get_or_insert_event_source()?;

    let pollfd = libc::pollfd {
        fd: (*event_source).unwrap().tty,
        events: libc::POLLIN,
        revents: 0,
    };

    let fds = vec![pollfd, pollfd].as_mut_ptr();

    if unsafe {
        libc::poll(
            fds,
            1 as libc::nfds_t,
            duration.as_millis() as libc::c_int,
        )
    } < 0
    {
        panic!("{}", std::io::Error::last_os_error());
    } else {
        Ok(())
    }
}
