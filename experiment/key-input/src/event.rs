const TTY_BUFFER_SIZE: usize = 1_024;

use crate::parser::{parse, KeyCode};
use crate::terminal::get_tty_file_descriptor;

struct EventSource {
    tty: i32,
    sig_winch: UnixStream,
}

use std::sync::{Mutex, MutexGuard};

static EVENT_SOURCE: Mutex<Option<EventSource>> = Mutex::new(None);

use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;

use signal_manipulator::self_pipe::register;

fn nonblocking_unix_pair() -> std::io::Result<(UnixStream, UnixStream)> {
    let (receiver, sender) = UnixStream::pair()?;
    receiver.set_nonblocking(true)?;
    sender.set_nonblocking(true)?;
    Ok((receiver, sender))
}

impl EventSource {
    fn new() -> std::io::Result<Self> {
        Ok(EventSource {
            tty: get_tty_file_descriptor()?,
            sig_winch: {
                let (receiver, sender) = nonblocking_unix_pair()?;

                register(libc::SIGWINCH, sender);

                receiver
            },
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
            let err = std::io::Error::last_os_error();
            match err.kind() {
                std::io::ErrorKind::Interrupted => Ok(1),
                _ => panic!("{}", err),
            }
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

    let result = event_source.as_ref().unwrap().read(&mut buffer)?;

    let key_code = parse(&buffer[..(result as usize)])?;

    Ok(key_code)
}

pub fn poll(duration: core::time::Duration) -> std::io::Result<()> {
    let event_source = get_or_insert_event_source()?;

    let pollfd = libc::pollfd {
        fd: event_source.as_ref().unwrap().tty,
        events: libc::POLLIN,
        revents: 0,
    };

    let pollfd2 = libc::pollfd {
        fd: event_source.as_ref().unwrap().sig_winch.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    let mut fds = [pollfd, pollfd2];

    if unsafe { crate::poll::poll(&mut fds, duration).is_err() } {
        let err = std::io::Error::last_os_error();
        match err.kind() {
            std::io::ErrorKind::Interrupted => Ok(()),
            _ => panic!("{}", err),
        }
    } else {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!(
            "IN {:?} {:?}",
            fds[0].revents == libc::POLLIN,
            fds[1].revents == libc::POLLIN
        );
        println!(
            "OUT {:?} {:?}",
            fds[0].revents == libc::POLLOUT,
            fds[1].revents == libc::POLLOUT
        );
        println!(
            "ERR {:?} {:?}",
            fds[0].revents == libc::POLLERR,
            fds[1].revents == libc::POLLERR
        );
        Ok(())
    }
}
