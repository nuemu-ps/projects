use terminal_manipulator::{
    cursor::{Hide, MoveTo},
    queue,
    style::Print,
    terminal::{enter_raw_mode, window_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    traits::Command,
    parser::{KeyCode},
    event::{poll, read, Event},
};

use std::io::Write;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    queue!(&mut stdout, EnterAlternateScreen);
    queue!(&mut stdout, Hide);

    enter_raw_mode()?;

    let now = Instant::now();

    loop {
        let window = window_size().unwrap();
        queue!(
            &mut stdout,
            Clear(ClearType::All),
            MoveTo(window.rows / 2, window.cols / 2 - 10),
            Print(format!("Times Elapsed: {:?}", now.elapsed().as_secs())),
            MoveTo(window.rows / 2 + 2, window.cols / 2 - 11),
            Print(format!("Press ESC to Close.")),
        );
        stdout.flush()?;

        if poll(Some(Duration::new(1, 0)))? {
            match read()? {
                Event::KeyPress(key_code) => {
                    if key_code == KeyCode::Esc {
                        break;
                    }
                },
                Event::WindowResize(_window_size) => {}
            }
        }
    }

    queue!(&mut stdout, LeaveAlternateScreen);
    stdout.flush()?;

    Ok(())
}
