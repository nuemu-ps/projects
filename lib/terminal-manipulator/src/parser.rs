#[derive(PartialEq)]
pub enum KeyCode {
    Esc,
    Left,
    Right,
    Up,
    Down,
    Something,
}

pub fn parse(buffer: &[u8]) -> std::io::Result<Option<KeyCode>> {
    match buffer[0] {
        b'\x1B' => {
            if buffer.len() == 0 {
                Ok(Some(KeyCode::Esc))
            } else {
                match buffer[1] {
                    b'O' => {
                        if buffer.len() == 2 {
                            Ok(None)
                        } else {
                            match buffer[2] {
                                b'D' => Ok(Some(KeyCode::Left)),
                                b'C' => Ok(Some(KeyCode::Right)),
                                b'A' => Ok(Some(KeyCode::Up)),
                                b'B' => Ok(Some(KeyCode::Down)),
                                _ => Ok(None)
                            }
                        }
                    },
                    b'[' => {
                        match buffer[2] {
                            b'D' => Ok(Some(KeyCode::Left)),
                            b'C' => Ok(Some(KeyCode::Right)),
                            b'A' => Ok(Some(KeyCode::Up)),
                            b'B' => Ok(Some(KeyCode::Down)),
                            _ => Ok(None)
                        }
                    },
                    _ => Ok(None)
                }
            }
        },
        _ => Ok(Some(KeyCode::Something)),
    }
}
