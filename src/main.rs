use nannou::{
    image::{DynamicImage, RgbImage},
    prelude::*,
};

struct Model {}
impl Model {
    fn new(app: &App) -> Self {
        // Create a new window
        app.new_window().size(512, 512).view(view).build().unwrap();
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
    let scale_factor = app.main_window().scale_factor();

    let win = app.window_rect();
    let mut img_buffer = RgbImage::new(512, 512);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = [255, 0, 0].into();
    }

    let dynamic_image = DynamicImage::ImageRgb8(img_buffer);
    let texture = wgpu::Texture::from_image(app, &dynamic_image);

    let draw = app.draw();
    draw.texture(&texture);

    draw.to_frame(app, &frame).unwrap();
}
