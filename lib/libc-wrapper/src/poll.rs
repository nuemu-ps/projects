pub fn poll(fds: &mut [Pollfd], duration: Option<core::time::Duration>) -> std::io::Result<usize> {
    poll_impl(fds, duration)
}

#[cfg(target_os = "linux")]
fn poll_impl(fds: &mut [Pollfd], duration: Option<core::time::Duration>) -> std::io::Result<usize> {
    let res = unsafe {
        libc::poll(
            fds.as_mut_ptr(),
            1 as libc::nfds_t,
            duration.map(|d| d.as_millis() as libc::c_int).unwrap_or(std::ptr::null_mut()),
        )
    };

    if res < 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(res)
    }
}

/*
    There may exist some bug in MacOS poll implementation.
*/
#[cfg(target_os = "macos")]
use crate::{
    Pollfd,
    POLLERR,
    POLLIN,
    POLLOUT,
    FdSet,
    Timeval,
    select,
};

#[cfg(target_os = "macos")]
fn poll_impl(fds: &mut [Pollfd], duration: Option<core::time::Duration>) -> std::io::Result<usize> {
    let mut read_set = FdSet::new();
    let mut write_set = FdSet::new();
    let mut except_set = FdSet::new();

    let mut nfds = 0;
    let timeval = duration.map(|d| Timeval {
        tv_sec: d.as_secs() as _,
        tv_usec: d.subsec_micros() as _,
    });

    for fd in fds.iter_mut() {
        fd.revents = 0;
        nfds = nfds.max(fd.fd);

        read_set.insert(fd.fd);
        write_set.insert(fd.fd);
        except_set.insert(fd.fd);
    }

    let res = select(
            (nfds + 1) as usize,
            &mut read_set,
            &mut write_set,
            &mut except_set,
            timeval
        );

    if res.is_ok() {
        for fd in fds.iter_mut() {
            if read_set.contains(fd.fd) {
                fd.revents |= POLLIN;
            }
            if write_set.contains(fd.fd) {
                fd.revents |= POLLOUT;
            }
            if except_set.contains(fd.fd) {
                fd.revents |= POLLERR;
            }
        }
    }

    res
}
