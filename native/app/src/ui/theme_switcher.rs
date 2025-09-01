use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct ThemeSwitcher {
    pub open: bool,
    pub items: Vec<String>,
    pub selected: usize,
    pub scroll: f32,
    last_toggle: Instant,
}

#[derive(Clone, Copy, Debug)]
pub struct OverlayLayout {
    pub w: f32,
    pub h: f32,
    pub row_h: f32,
    pub padding: f32,
    pub max_rows: usize,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RowRenderItem<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub selected: bool,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self {
            open: false,
            items: Vec::new(),
            selected: 0,
            scroll: 0.0,
            last_toggle: Instant::now(),
        }
    }

    pub fn open_with(&mut self, items: Vec<String>) {
        self.items = items;
        if self.items.is_empty() {
            self.open = false;
            return;
        }
        self.selected = self.selected.min(self.items.len().saturating_sub(1));
        self.open = true;
        self.snap_to_selection(OverlayLayout::defaults());
    }

    pub fn toggle(&mut self, items_provider: impl FnOnce() -> Vec<String>) {
        let now = Instant::now();
        if now.duration_since(self.last_toggle) < Duration::from_millis(120) {
            return;
        }
        self.last_toggle = now;
        if self.open {
            self.open = false;
        } else {
            self.open_with(items_provider());
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn current(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }

    pub fn handle_key(&mut self, key: Key) -> Action {
        if !self.open {
            return Action::None;
        }
        match key {
            Key::Escape => {
                self.open = false;
                Action::Close
            }
            Key::Enter => {
                self.open = false;
                Action::Apply(self.current().unwrap_or_default().to_string())
            }
            Key::Up => {
                self.move_sel(-1);
                Action::Moved
            }
            Key::Down => {
                self.move_sel(1);
                Action::Moved
            }
        }
    }

    pub fn handle_key_page(&mut self, pg: Page) -> Action {
        if !self.open {
            return Action::None;
        }
        match pg {
            Page::Up => {
                self.page(-1);
                Action::Moved
            }
            Page::Down => {
                self.page(1);
                Action::Moved
            }
        }
    }

    pub fn layout(&mut self, viewport_w: f32, viewport_h: f32) -> OverlayLayout {
        let w = (viewport_w * 0.38).clamp(420.0, 720.0);
        let h = (viewport_h * 0.50).clamp(300.0, 560.0);
        let row_h = 28.0;
        let padding = 16.0;
        let max_rows = ((h - padding * 3.0 - 28.0) / row_h).floor().max(1.0) as usize;
        OverlayLayout {
            w,
            h,
            row_h,
            padding,
            max_rows,
        }
    }

    pub fn render_items<'a>(
        &'a mut self,
        layout: &OverlayLayout,
        viewport_w: f32,
        viewport_h: f32,
    ) -> (OverlayBox, Vec<RowRenderItem<'a>>) {
        let top_idx = self.visible_top_index(layout);
        let x0 = (viewport_w - layout.w) * 0.5;
        let y0 = (viewport_h - layout.h) * 0.5;

        let mut rows = Vec::with_capacity(layout.max_rows);
        let visible = self
            .items
            .iter()
            .enumerate()
            .skip(top_idx)
            .take(layout.max_rows);

        for (i, s) in visible {
            let row_y = y0 + layout.padding + 28.0 + (i - top_idx) as f32 * layout.row_h
                - self.scroll.fract() * layout.row_h;
            rows.push(RowRenderItem {
                text: s.as_str(),
                x: x0 + layout.padding * 2.0,
                y: row_y,
                selected: i == self.selected,
            });
        }

        (
            OverlayBox {
                x: x0,
                y: y0,
                w: layout.w,
                h: layout.h,
            },
            rows,
        )
    }

    fn move_sel(&mut self, delta: isize) {
        if self.items.is_empty() {
            return;
        }
        let len = self.items.len();
        let cur = self.selected as isize;
        let mut next = cur + delta;
        if next < 0 {
            next = 0;
        }
        if next >= len as isize {
            next = len as isize - 1;
        }
        self.selected = next as usize;
        self.snap_to_selection(OverlayLayout::defaults());
    }

    fn page(&mut self, dir: isize) {
        let lay = OverlayLayout::defaults();
        let jump = lay.max_rows as isize - 1;
        self.move_sel(jump * dir);
    }

    fn snap_to_selection(&mut self, layout: OverlayLayout) {
        let sel = self.selected as isize;
        let top = self.visible_top_index(&layout) as isize;
        let bottom = top + layout.max_rows as isize - 1;
        let mut scroll_rows = 0isize;
        if sel < top {
            scroll_rows = sel - top;
        }
        if sel > bottom {
            scroll_rows = sel - bottom;
        }
        self.scroll = (self.scroll + scroll_rows as f32)
            .clamp(0.0, (self.items.len().saturating_sub(1)) as f32);
    }

    fn visible_top_index(&self, layout: &OverlayLayout) -> usize {
        let max_top = self.items.len().saturating_sub(layout.max_rows);
        self.scroll.floor().clamp(0.0, max_top as f32) as usize
    }
}

impl OverlayLayout {
    pub fn defaults() -> Self {
        OverlayLayout {
            w: 560.0,
            h: 420.0,
            row_h: 28.0,
            padding: 16.0,
            max_rows: 12,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct OverlayBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Key {
    Up,
    Down,
    Enter,
    Escape,
}
#[derive(Clone, Copy, Debug)]
pub enum Page {
    Up,
    Down,
}

#[derive(Debug)]
pub enum Action {
    None,
    Moved,
    Close,
    Apply(String),
}
