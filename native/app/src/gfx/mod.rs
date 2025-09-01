#[allow(dead_code)]
pub const GRID_SHADER: &str = include_str!("shaders/grid.wgsl");

pub mod renderer;
pub mod text;

pub use renderer::Renderer;
