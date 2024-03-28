const TTY_BUFFER_SIZE: usize = 1_024;

fn get_file_descriptor() -> std::io::Result<i32> {
    let fd = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        libc::STDIN_FILENO
    } else {
        panic!();
    };

    Ok(fd)
}

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
    };

    unsafe { libc::tcgetattr(fd, &mut termios) };
    unsafe { libc::cfmakeraw(&mut termios) };
    unsafe { libc::tcsetattr(fd, 0, &mut termios) };

    Ok(())
}

fn main() -> std::io::Result<()> {
    enter_raw_mode()?;

    let fd = get_file_descriptor()?;

    let mut buffer = [0u8; TTY_BUFFER_SIZE];

    let mut pollfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };

    loop {
        if unsafe { libc::poll(&mut pollfd, 1 as libc::nfds_t, 1000 as libc::c_int) } < 0 {
            panic!()
        }
        let result = unsafe {
            libc::read(
                fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len() as libc::size_t,
            )
        };
        if result < 0 {
            println!("error");
        }
        let input_buffer = &buffer[..(result as usize)];

        let mut input = String::from("");

        print!("{}", std::str::from_utf8(input_buffer).unwrap());

        if buffer[0] == b'\x1B' { // ESC
            break;
        }
    }

    Ok(())
}
