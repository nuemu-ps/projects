pub struct Timeval {
    pub tv_sec: usize,
    pub tv_usec: usize,
}

impl Timeval {
    pub fn as_libc_val(&self) -> libc::timeval {
        libc::timeval {
            tv_sec: self.tv_sec as _,
            tv_usec: self.tv_usec as _,
        }
    }
}
