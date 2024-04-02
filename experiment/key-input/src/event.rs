const TTY_BUFFER_SIZE: usize = 1_024;

use crate::parser::{parse, KeyCode};
use crate::terminal::get_tty_file_descriptor;

struct EventSource {
    tty: i32,
    sig_winch: UnixStream,
}

use std::sync::{Mutex, MutexGuard};

static EVENT_SOURCE: Mutex<Option<EventSource>> = Mutex::new(None);

use std::os::unix::net::UnixStream;
use std::os::fd::AsRawFd;
use std::ops::Deref;

static SIGNAL_SOURCE: Mutex<Option<i32>> = Mutex::new(None);

extern "C" fn handler(_sig: libc::c_int, _info: *mut libc::siginfo_t, _data: *mut libc::c_void) {
    // let signal_source = get_or_insert_signal_source(None).unwrap();
    // let data = b"X" as *const _ as *const _;

    // println!("{}", signal_source.deref().unwrap());
    // unsafe { libc::write(signal_source.deref().unwrap(), data, 1) };
}

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

                // drop(get_or_insert_signal_source(Some(sender.as_raw_fd()))?);

                let mut new: libc::sigaction = unsafe { core::mem::zeroed() };
                new.sa_sigaction = handler as usize;
                new.sa_flags = libc::SA_SIGINFO;
                let mut old: libc::sigaction = unsafe { core::mem::zeroed() };

                unsafe { libc::sigaction(libc::SIGWINCH, &new, &mut old) };

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
                _ => panic!("{}", err)
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

fn get_or_insert_signal_source(fd: Option<i32>) -> std::io::Result<MutexGuard<'static, Option<i32>>> {
    if let Ok(mut optional_signal_source) = SIGNAL_SOURCE.lock() {
        if optional_signal_source.is_none() && fd.is_some() {
            *optional_signal_source = Some(fd.unwrap());
        }
        return Ok(optional_signal_source);
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

    // let pollfd2 = libc::pollfd {
    //     fd: event_source.as_ref().unwrap().sig_winch.as_raw_fd(),
    //     events: libc::POLLIN,
    //     revents: 0,
    // };

    if unsafe {
        libc::poll(
            [pollfd].as_mut_ptr(),
            1 as libc::nfds_t,
            duration.as_millis() as libc::c_int,
        )
    } < 0
    {
        let err = std::io::Error::last_os_error();
        match err.kind() {
            std::io::ErrorKind::Interrupted => Ok(()),
            _ => panic!("{}", err)
        }
    } else {
        Ok(())
    }
}
