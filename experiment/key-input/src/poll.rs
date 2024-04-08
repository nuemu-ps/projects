struct FdSet {
    set: libc::fd_set,
}

impl FdSet {
    fn new() -> Self {
        unsafe {
            let mut set = std::mem::MaybeUninit::<libc::fd_set>::uninit();
            libc::FD_ZERO(set.as_mut_ptr());
            Self {
                set: set.assume_init(),
            }
        }
    }

    fn add(&mut self, fd: std::os::unix::io::RawFd) {
        unsafe {
            libc::FD_SET(fd, &mut self.set);
        }
    }

    fn contains(&mut self, fd: std::os::unix::io::RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.set) }
    }
}

// Non Mac OS ?
// libc::poll(
//     fds.as_mut_ptr(),
//     1 as libc::nfds_t,
//     duration.as_millis() as libc::c_int,
// )

pub fn poll(fds: &mut [libc::pollfd], duration: core::time::Duration) -> std::io::Result<isize> {
    let mut read_set = FdSet::new();
    let mut write_set = FdSet::new();
    let mut except_set = FdSet::new();

    let mut nfds = 0;

    for fd in fds.iter_mut() {
        fd.revents = 0;
        nfds = nfds.max(fd.fd);

        read_set.add(fd.fd);
        write_set.add(fd.fd);
        except_set.add(fd.fd);
    }

    let res = unsafe {
        libc::select(
            nfds + 1,
            &mut read_set.set,
            &mut write_set.set,
            &mut except_set.set,
            &mut libc::timeval {
                tv_sec: duration.as_secs() as _,
                tv_usec: duration.subsec_micros() as _,
            } as *mut _,
        )
    };

    if res < 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        for fd in fds.iter_mut() {
            if read_set.contains(fd.fd) {
                fd.revents |= libc::POLLIN;
            }
            if write_set.contains(fd.fd) {
                fd.revents |= libc::POLLOUT;
            }
            if except_set.contains(fd.fd) {
                fd.revents |= libc::POLLERR;
            }
        }

        Ok(res as isize)
    }
}
