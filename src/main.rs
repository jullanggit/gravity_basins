use wgpu::include_spirv;

fn main() {
    let shader = include_spirv!(env!("shader_test.spv"));
    println!("Hello, world!");
}
