/*
    fd_set
    ref: https://www.man7.org/linux/man-pages/man2/select.2.html

    note: downgraded copy of  https://docs.rs/nix/latest/nix/sys/select/struct.FdSet.html
*/

use std::os::unix::io::RawFd;

pub struct FdSet {
    pub set: libc::fd_set,
}

impl FdSet {
    pub fn new() -> Self {
        unsafe {
            let mut set = std::mem::MaybeUninit::<libc::fd_set>::uninit();
            libc::FD_ZERO(set.as_mut_ptr());
            Self {
                set: set.assume_init(),
            }
        }
    }

    pub fn insert(&mut self, fd: RawFd) {
        unsafe {
            libc::FD_SET(fd, &mut self.set);
        }
    }

    pub fn remove(&mut self, fd: RawFd) {
        unsafe {
            libc::FD_CLR(fd, &mut self.set);
        }
    }

    pub fn contains(&mut self, fd: RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.set) }
    }
}
