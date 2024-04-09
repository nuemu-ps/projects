/*
    select(2)
    ref: https://www.man7.org/linux/man-pages/man2/select.2.html
*/

use crate::{FdSet, Timeval};
use std::io::{Error, Result};

pub fn select(
    nfds: usize,
    read_set: &mut FdSet,
    write_set: &mut FdSet,
    except_set: &mut FdSet,
    timeout: Option<Timeval>,
) -> Result<usize> {
    let res = unsafe {
        libc::select(
            nfds as i32,
            &mut read_set.set,
            &mut write_set.set,
            &mut except_set.set,
            timeout.map(|timeval| &mut timeval.as_libc_val() as *mut _ as *mut libc::timeval).unwrap_or(std::ptr::null_mut()),
        )
    };

    if res < 0 {
        Err(Error::last_os_error().into())
    } else {
        Ok(res as usize)
    }
}
