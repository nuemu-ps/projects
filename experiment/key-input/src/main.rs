const TTY_BUFFER_SIZE: usize = 1_024;

fn enter_raw_mode() -> std::io::Result<()> {
    let fd = get_file_descriptor()?;

    let mut termios = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    }; // TODO: learn termios

    unsafe { libc::tcgetattr(fd, &mut termios) };
    unsafe { libc::cfmakeraw(&mut termios) };
    unsafe { libc::tcsetattr(fd, 0, &mut termios) };

    Ok(())
}

struct FileDescriptor (i32);

impl FileDescriptor {
    fn new() -> std::io::Result<Self> {
        Ok(FileDescriptor(get_file_descriptor()?))
    }

    pub fn read(&self, buffer: &mut [u8]) -> std::io::Result<isize> {
        let result = unsafe {libc::read(
            self.0,
            buffer.as_mut_ptr() as *mut libc::c_void,
            buffer.len() as libc::size_t,
        )};

        if result < 0 {
            panic!("{}", std::io::Error::last_os_error());
        } else {
            Ok(result)
        }
    }
}

fn get_file_descriptor() -> std::io::Result<i32> {
    let fd = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        libc::STDIN_FILENO
    } else {
        panic!();
    };

    Ok(fd)
}

fn poll(file_descriptor: &FileDescriptor, duration: core::time::Duration) -> std::io::Result<()> {
    let mut pollfd = libc::pollfd {
        fd: file_descriptor.0,
        events: libc::POLLIN,
        revents: 0,
    };

    if unsafe { libc::poll(&mut pollfd, 1 as libc::nfds_t, duration.as_millis() as libc::c_int) } < 0 {
        panic!("{}", std::io::Error::last_os_error());
    } else {
        Ok(())
    }
}

#[derive(PartialEq)]
enum KeyCode {
    Esc,
    Something
}


fn parse(buffer: &[u8]) -> std::io::Result<KeyCode> {
    match buffer[0] {
        b'\x1B' => Ok(KeyCode::Esc),
        _ => Ok(KeyCode::Something)
    }
}

fn read(file_descriptor: &FileDescriptor) -> std::io::Result<KeyCode> {
    let mut buffer = [0u8; TTY_BUFFER_SIZE];

    let result = file_descriptor.read(&mut buffer)?;

    let key_code = parse(&buffer[..(result as usize)])?;

    Ok(key_code)
}

fn main() -> std::io::Result<()> {
    enter_raw_mode()?;

    let fd = FileDescriptor::new()?;

    loop {
        if poll(&fd, core::time::Duration::new(1, 0)).is_ok() {
            let key_code = read(&fd)?;

            if key_code == KeyCode::Esc {
                break;
            }
        }
    }

    Ok(())
}
