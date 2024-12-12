use nannou::{
    image::{DynamicImage, RgbImage},
    prelude::*,
};

struct Model {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}
impl Model {
    fn new(app: &App) -> Self {
        // Create a new window
        let window_id = app.new_window().fullscreen().view(view).build().unwrap();
        let window = app.window(window_id).unwrap();
        let device = window.device();
        let format = Frame::TEXTURE_FORMAT;

        let vs_desc = wgpu::include_wgsl!("vertex.wgsl");
        let fs_desc = wgpu::include_wgsl!("fragment.wgsl");
        Self {}
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

// Vertices of a rectangle
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

fn main() {
    nannou::app(Model::new)
        .event(event)
        .simple_window(view)
        .run();
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win = app.window_rect();
    let width = win.w().ceil() as u32;
    let height = win.h().ceil() as u32;

    let mut img_buffer = RgbImage::new(width, height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = [255, 0, 0].into();
    }

    let dynamic_image = DynamicImage::ImageRgb8(img_buffer);
    let texture = wgpu::Texture::from_image(app, &dynamic_image);

    let draw = app.draw();
    draw.texture(&texture);

    draw.to_frame(app, &frame).unwrap();
}
