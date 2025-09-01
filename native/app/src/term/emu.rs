
pub struct Emu {
    lines: Vec<String>,
    cols: usize,
}

impl Emu {
    pub fn new(cols: u16, rows: u16) -> Self {
        Emu {
            lines: vec![String::new(); rows as usize],
            cols: cols as usize,
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        for &b in bytes {
            match b {
                b'\n' => self.lines.push(String::new()),
                b'\r' => {
                    if let Some(last) = self.lines.last_mut() {
                        last.clear();
                    }
                }
                0x08 => {
                    if let Some(last) = self.lines.last_mut() {
                        last.pop();
                    }
                }
                _ => {
                    if let Some(last) = self.lines.last_mut() {
                        last.push(b as char);
                        if last.len() >= self.cols {
                            self.lines.push(String::new());
                        }
                    }
                }
            }
        }
        if self.lines.len() > 1000 {
            let overflow = self.lines.len() - 1000;
            self.lines.drain(0..overflow);
        }
    }

    pub fn resize(&mut self, cols: u16, _rows: u16) {
        self.cols = cols as usize;
    }

    pub fn snapshot(&self, rows: usize) -> String {
        let start = self.lines.len().saturating_sub(rows);
        self.lines[start..].join("\n")
    }
=======
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
