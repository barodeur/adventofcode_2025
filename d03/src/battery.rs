#[derive(Debug)]
pub struct Battery(u8);

impl Battery {
    pub fn joltage(&self) -> u8 {
        self.0
    }

    pub fn parse_char(c: char) -> Battery {
        Battery(c as u8 - b'0')
    }
}
