use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;
use vte::{Params, Parser, Perform};

#[derive(Clone, Copy, Default)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Rgba,
    pub bg: Rgba,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', fg: Rgba(0xD6,0xEF,0xFF,0xFF), bg: Rgba(0x07,0x12,0x1A,0xFF) }
    }
}

pub struct Emu {
    pub cols: usize,
    pub rows: usize,
    pub grid: Vec<Cell>,
    pub cur_x: usize,
    pub cur_y: usize,
    parser: Parser,
    cur_fg: Rgba,
    cur_bg: Rgba,
}

impl Emu {
    pub fn new(cols: usize, rows: usize) -> Self {
        let cols = max(1, cols);
        let rows = max(1, rows);
        Self {
            cols,
            rows,
            grid: vec![Cell::default(); cols * rows],
            cur_x: 0,
            cur_y: 0,
            parser: Parser::new(),
            cur_fg: Cell::default().fg,
            cur_bg: Cell::default().bg,
        }
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = max(1, cols);
        self.rows = max(1, rows);
        self.grid = vec![Cell::default(); self.cols * self.rows];
        self.cur_x = 0;
        self.cur_y = 0;
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize { y * self.cols + x }

    pub fn draw_char(&mut self, c: char) {
        if c == '\n' {
            self.cur_x = 0;
            if self.cur_y + 1 >= self.rows { self.scroll_up(); } else { self.cur_y += 1; }
            return;
        }
        if c == '\r' { self.cur_x = 0; return; }
        if c == '\x08' { // BS
            if self.cur_x > 0 { self.cur_x -= 1; }
            return;
        }
        if c.is_control() { return; }
        let w = UnicodeWidthChar::width(c).unwrap_or(1).max(1);
        if self.cur_x + w > self.cols {
            self.cur_x = 0;
            if self.cur_y + 1 >= self.rows { self.scroll_up(); } else { self.cur_y += 1; }
        }
        let i = self.idx(self.cur_x, self.cur_y);
        self.grid[i] = Cell { ch: c, fg: self.cur_fg, bg: self.cur_bg };
        self.cur_x += w;
    }

    fn clear_all(&mut self) {
        self.grid.fill(Cell::default());
        self.cur_x = 0; self.cur_y = 0;
    }

    fn scroll_up(&mut self) {
        if self.rows <= 1 { return; }
        let w = self.cols;
        let len = self.grid.len();
        self.grid.copy_within(w.., 0);
        for c in self.grid[(len - w)..].iter_mut() { *c = Cell::default(); }
    }

    /// Feed raw PTY bytes: use vte to parse ANSI and print UTF-8 safely.
    pub fn on_bytes(&mut self, bytes: &[u8]) {
        let mut parser = std::mem::take(&mut self.parser);
        for &b in bytes {
            parser.advance(self, b);
        }
        self.parser = parser;
    }
}

impl Perform for Emu {
    fn print(&mut self, c: char) { self.draw_char(c); }
    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.draw_char('\n'),
            b'\r' => self.draw_char('\r'),
            b'\x08' => self.draw_char('\x08'),
            _ => {}
        }
    }
    fn csi_dispatch(&mut self, params: &Params, _ints: &[u8], _ignore: bool, action: char) {
        match action {
            'H' | 'f' => {
                let mut iter = params.iter();
                let y = iter.next().and_then(|p| p.get(0)).copied().unwrap_or(1);
                let x = iter.next().and_then(|p| p.get(0)).copied().unwrap_or(1);
                let x = x.saturating_sub(1) as usize;
                let y = y.saturating_sub(1) as usize;
                self.cur_x = min(x, self.cols.saturating_sub(1));
                self.cur_y = min(y, self.rows.saturating_sub(1));
            }
            'J' => {
                if *params.iter().next().and_then(|p| p.get(0)).unwrap_or(&0) == 2 {
                    self.clear_all();
                }
            }
            'm' => {
                if *params.iter().next().and_then(|p| p.get(0)).unwrap_or(&0) == 0 {
                    self.cur_fg = Cell::default().fg;
                    self.cur_bg = Cell::default().bg;
                }
            }
            _ => {}
        }
    }
}

