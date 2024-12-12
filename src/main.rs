use nannou::{
    image::{DynamicImage, RgbImage},
    prelude::*,
};

struct Model {}
impl Model {
    fn new(app: &App) -> Self {
        // Create a new window
        app.new_window().fullscreen().view(view).build().unwrap();
        Self {}
    }
}

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
