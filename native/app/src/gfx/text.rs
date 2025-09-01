use anyhow::{anyhow, Result};
use fontdb::{Database, Family, Query};
use wgpu::util::StagingBelt;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

pub struct TextLayer {
    pub brush: GlyphBrush<()>,
    pub belt: StagingBelt,
}

impl TextLayer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self> {
        let mut db = Database::new();
        db.load_system_fonts();
        let query = Query {
            families: &[Family::Name("JetBrains Mono"), Family::Monospace],
            ..Query::default()
        };
        let id = db
            .query(&query)
            .ok_or_else(|| anyhow!("no monospace font found"))?;
        let data = db
            .with_face_data(id, |d, _| d.to_vec())
            .ok_or_else(|| anyhow!("font data unavailable"))?;
        let font = ab_glyph::FontArc::try_from_vec(data)?;
        let brush = GlyphBrushBuilder::using_font(font).build(device, format);
        Ok(Self {
            brush,
            belt: StagingBelt::new(1024),
        })
    }

    pub fn queue(&mut self, text: &str, width: f32, height: f32) {
        let section = Section {
            screen_position: (20.0, 30.0),
            text: vec![Text::new(text)
                .with_color([0.9, 0.95, 1.0, 1.0])
                .with_scale(18.0)],
            bounds: (width, height),
            ..Section::default()
        };
        self.brush.queue(section);
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.brush
            .draw_queued(device, &mut self.belt, encoder, view, width, height)
            .map_err(|e| anyhow!(e))?;
        self.belt.finish();
        Ok(())
    }
}
