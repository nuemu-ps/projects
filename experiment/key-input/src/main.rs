use key_input::event::{poll, read};
use key_input::parser::KeyCode;
use key_input::terminal::enter_raw_mode;

fn main() -> std::io::Result<()> {
    enter_raw_mode()?;

    loop {
        if poll(Some(core::time::Duration::new(10, 0))).is_ok() {
            println!("poll");
            let key_code = read()?;

            if key_code == KeyCode::Esc {
                break;
            }
        }
    }

    Ok(())
}
