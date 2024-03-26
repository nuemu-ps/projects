use terminal_manipulator::{
    queue,
    cursor::{MoveTo, Hide},
    style::Print,
    terminal::{
        window_size,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        Clear,
        ClearType
    },
    traits::Command
};

use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    queue!(&mut stdout, EnterAlternateScreen)?;
    queue!(&mut stdout, Hide)?;

    let now = Instant::now();

    loop {
        let window = window_size().unwrap();
        queue!(&mut stdout,
            Clear(ClearType::All),
            MoveTo(
                (window.rows / 2),
                (window.columns / 2 - 10)
            ),
            Print(format!("Times Elapsed: {:?}", now.elapsed().as_secs())),
        );
        stdout.flush()?;

        thread::sleep(Duration::from_millis(1));

        if now.elapsed().as_secs() == 11 {
            break;
        }
    }

    queue!(&mut stdout, LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}
