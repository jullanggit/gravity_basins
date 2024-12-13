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
impl AppState {
    /// Converts the app state to wgsl bytes
    fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self)?;
        Ok(buffer.into_inner())
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            pos_x: 0.,
            pos_y: 0.,
            zoom: 1.,
            // TODO: figure out how to initialise this
            gravitons_length: ArrayLength,
            gravitons: Vec::new(),
        }
    }
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
