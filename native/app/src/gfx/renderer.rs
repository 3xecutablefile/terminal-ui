use wgpu::{util::StagingBelt, CommandEncoder, Device, Queue, TextureFormat, TextureView};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

use crate::theme::Theme;
use crate::ui::theme_switcher::{OverlayBox, RowRenderItem};

// If panels are placed elsewhere, adjust the path accordingly.
#[allow(unused_imports)]
use crate::ui::panels::Panels;

pub struct Renderer {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
}

impl Renderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/DejaVuSansMono.ttf"
        ))
        .expect("font slice");
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(device, format);
        let staging_belt = StagingBelt::new(1024);
        Self {
            glyph_brush,
            staging_belt,
        }
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {}

    pub fn draw_theme_overlay_begin(
        &mut self,
        _enc: &mut CommandEncoder,
        _boxr: OverlayBox,
        _theme: &Theme,
    ) {
    }

    pub fn draw_theme_overlay_rows(
        &mut self,
        _enc: &mut CommandEncoder,
        _rows: &[RowRenderItem],
        _theme: &Theme,
    ) {
    }

    // === Panel primitives ===
    #[allow(clippy::too_many_arguments)]
    pub fn draw_rounded_rect(
        &mut self,
        _enc: &mut CommandEncoder,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
        _fill: &str,
        _stroke: &str,
        _stroke_w: f32,
    ) {
        // TODO: implement shape rendering
    }

    pub fn draw_text(&mut self, x: f32, y: f32, text: &str, color: &str, px: f32) {
        let color = self.parse_color(color);
        self.glyph_brush.queue(Section {
            screen_position: (x, y),
            text: vec![Text::new(text).with_color(color).with_scale(px)],
            ..Section::default()
        });
    }

    pub fn flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        mut enc: CommandEncoder,
        view: &TextureView,
        width: u32,
        height: u32,
    ) {
        if let Err(e) = self.glyph_brush.draw_queued(
            device,
            &mut self.staging_belt,
            &mut enc,
            view,
            width,
            height,
        ) {
            log::error!("glyph_brush error: {e}");
        }
        self.staging_belt.finish();
        queue.submit(Some(enc.finish()));
        self.staging_belt.recall();
    }

    pub fn draw_side_panel(
        &mut self,
        enc: &mut CommandEncoder,
        x: f32,
        w: f32,
        h: f32,
        theme: &Theme,
    ) {
        self.draw_rounded_rect(
            enc,
            x,
            0.0,
            w,
            h,
            &theme.ui.panel_bg,
            &theme.ui.panel_border,
            1.0,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar(
        &mut self,
        enc: &mut CommandEncoder,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        value: f32,
        label: &str,
        theme: &Theme,
    ) {
        let value = value.clamp(0.0, 1.0);
        self.draw_rounded_rect(enc, x, y, w, h, "rgba(255,255,255,0.06)", "", 0.0);
        let fw = w * value;
        self.draw_rounded_rect(enc, x, y, fw, h, &theme.ui.accent, "", 0.0);
        self.draw_text(x, y - 8.0, label, &theme.ui.text, 12.0);
    }

    // === Ambient effects ===
    pub fn draw_neon_grid(&mut self, _enc: &mut CommandEncoder, _w: u32, _h: u32, _theme: &Theme) {
        // TODO: implement neon grid shader using theme.effects.grid_color and grid_spacing
    }

    pub fn draw_scanlines(&mut self, _enc: &mut CommandEncoder, _w: u32, _h: u32, _theme: &Theme) {
        // TODO: implement scanline overlay using theme.effects.scanline_opacity
    }

    #[allow(dead_code)]
    pub fn parse_color(&self, s: &str) -> [f32; 4] {
        fn clamp(v: f32) -> f32 {
            v.clamp(0.0, 1.0)
        }
        let t = s.trim();
        if let Some(hex) = t.strip_prefix('#') {
            let (r, g, b, a) = match hex.len() {
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                    (r, g, b, 255)
                }
                8 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                    let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                    (r, g, b, a)
                }
                _ => (0, 0, 0, 255),
            };
            return [
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ];
        }
        if let Some(rest) = t.strip_prefix("rgba(").and_then(|x| x.strip_suffix(')')) {
            let parts: Vec<_> = rest.split(',').map(|p| p.trim()).collect();
            if parts.len() == 4 {
                let r: f32 = parts[0].parse().unwrap_or(0.0);
                let g: f32 = parts[1].parse().unwrap_or(0.0);
                let b: f32 = parts[2].parse().unwrap_or(0.0);
                let a: f32 = parts[3].parse().unwrap_or(1.0);
                return [
                    clamp(r / 255.0),
                    clamp(g / 255.0),
                    clamp(b / 255.0),
                    clamp(a),
                ];
            }
        }
        [1.0, 1.0, 1.0, 1.0]
    }
}
