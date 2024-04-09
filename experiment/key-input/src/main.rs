use key_input::event::{poll, read, Event};
use key_input::parser::KeyCode;
use key_input::terminal::enter_raw_mode;

fn main() -> std::io::Result<()> {
    enter_raw_mode()?;

    loop {
        if poll(Some(core::time::Duration::new(10, 0))).is_ok() {
            let event = read()?;

            match event {
                Event::KeyPress(key_code) => {
                    if key_code == KeyCode::Esc {
                        break;
                    }
                },
                Event::WindowResize => {
                    println!("Resize");
                }
            }
        }
    }

    Ok(())
}
