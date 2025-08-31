#[allow(dead_code)]
pub struct Emu;

#[allow(dead_code)]
impl Emu {
    pub fn new(_cols: u16, _rows: u16) -> Self {
        Emu
    }

    pub fn feed(&mut self, _bytes: &[u8]) {}

    pub fn resize(&mut self, _cols: u16, _rows: u16) {}
}
