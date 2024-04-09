use std::os::fd::RawFd;

// c_short <-> i32
type Poll = usize;

pub const POLLIN: Poll = 0x1;
pub const POLLOUT: Poll = 0x4;
pub const POLLERR: Poll = 0x8;

pub struct Pollfd {
    pub fd: RawFd,
    pub events: Poll,
    pub revents: Poll,
}

impl Pollfd {
    pub fn as_pollfd(&self) -> libc::pollfd {
        libc::pollfd {
            fd: self.fd,
            events: self.events as libc::c_short,
            revents: self.revents as libc::c_short,
        }
    }
}
