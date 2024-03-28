use key_input::event::{poll, read};
use key_input::parser::KeyCode;
use key_input::terminal::enter_raw_mode;

fn main() -> std::io::Result<()> {
    enter_raw_mode()?;

    loop {
        if poll(core::time::Duration::new(1, 0)).is_ok() {
            let key_code = read()?;

            if key_code == KeyCode::Esc {
                break;
            }
        }
    }

    Ok(())
}
