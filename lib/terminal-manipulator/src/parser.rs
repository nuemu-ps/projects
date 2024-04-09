#[derive(PartialEq)]
pub enum KeyCode {
    Esc,
    Something,
}

pub fn parse(buffer: &[u8]) -> std::io::Result<KeyCode> {
    match buffer[0] {
        b'\x1B' => Ok(KeyCode::Esc),
        _ => Ok(KeyCode::Something),
    }
}
