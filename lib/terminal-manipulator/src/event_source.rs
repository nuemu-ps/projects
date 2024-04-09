use crate::file_descriptor::FileDescritor;

pub struct EventSource {
    pub tty: FileDescritor,
    pub sig_winch: FileDescritor,
}

use crate::terminal::get_tty_file_descriptor;
use std::sync::{Mutex, MutexGuard};

static EVENT_SOURCE: Mutex<Option<EventSource>> = Mutex::new(None);

use std::os::unix::net::UnixStream;

use signal_manipulator::self_pipe::register;

use std::os::fd::IntoRawFd;

fn nonblocking_unix_pair() -> std::io::Result<(UnixStream, UnixStream)> {
    let (receiver, sender) = UnixStream::pair()?;
    receiver.set_nonblocking(true)?;
    sender.set_nonblocking(true)?;
    Ok((receiver, sender))
}

impl EventSource {
    fn new() -> std::io::Result<Self> {
        Ok(EventSource {
            tty: FileDescritor::new(get_tty_file_descriptor()?),
            sig_winch: {
                let (receiver, sender) = nonblocking_unix_pair()?;

                register(libc::SIGWINCH, sender);

                FileDescritor::new(receiver.into_raw_fd())
            },
        })
    }
}

pub fn get_or_insert_event_source() -> std::io::Result<MutexGuard<'static, Option<EventSource>>> {
    if let Ok(mut optional_event_source) = EVENT_SOURCE.lock() {
        if optional_event_source.is_none() {
            *optional_event_source = Some(EventSource::new()?);
        }
        return Ok(optional_event_source);
    }
    panic!("{}", std::io::Error::last_os_error())
}
