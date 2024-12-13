use encase::{ArrayLength, ShaderType};

#[derive(ShaderType)]
struct AppState {
    pos_x: f32,
    pos_y: f32,
    zoom: f32,
    gravitons_length: ArrayLength,
    #[size(runtime)]
    gravitons: Vec<Graviton>,
}

#[derive(ShaderType)]
struct Graviton {
    x: f32,
    y: f32,
    color: Color,
}

/// A RGB Color
#[derive(ShaderType)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

fn main() {}
