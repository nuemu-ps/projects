use std::os::fd::RawFd;

pub struct FileDescritor {
    pub fd: RawFd,
}

impl FileDescritor {
    pub fn new(fd: RawFd) -> Self {
        Self { fd }
    }

    pub fn read(&self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let result = libc_wrapper::read(self.fd, buffer, buffer.len());

        result
    }
}
