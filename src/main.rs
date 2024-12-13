struct AppState {
    pos_x: f32,
    pos_y: f32,
    zoom: f32,
    gravitons: Vec<Graviton>,
}

struct Graviton {
    x: f32,
    y: f32,
    color: Color,
}

/// A RGB Color
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

fn main() {}
