pub fn enter_raw_mode() -> std::io::Result<()> {
    let fd = get_tty_file_descriptor()?;

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

pub fn get_tty_file_descriptor() -> std::io::Result<i32> {
    let fd = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        libc::STDIN_FILENO
    } else {
        panic!();
    };

    Ok(fd)
}
