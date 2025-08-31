struct Params {
    spacing: f32,
    opacity: f32,
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> params: Params;

@vertex
fn vs(@builtin(vertex_index) vi:u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>,6>(
        vec2<f32>(-1.0,-1.0), vec2<f32>(1.0,-1.0), vec2<f32>(1.0,1.0),
        vec2<f32>(-1.0,-1.0), vec2<f32>(1.0,1.0), vec2<f32>(-1.0,1.0));
    return vec4<f32>(pos[vi],0.0,1.0);
}

@fragment
fn fs(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let x = pos.x / params.spacing;
    let y = pos.y / params.spacing;
    let gx = fract(x);
    let gy = fract(y);
    let grid_line = select(0.0, 1.0, gx < 0.02 || gy < 0.02);
    return params.color * (grid_line * params.opacity);
}
