use std::io::{Result, Error};
use std::os::fd::RawFd;

pub fn read(fd: RawFd, buf: &mut [u8], count: usize) -> Result<usize> {
    let res = unsafe {
        libc::read(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            count as libc::size_t,
        )
    };

    if res < 0 {
        Err(Error::last_os_error().into())
    } else {
        Ok(res as usize)
    }
}