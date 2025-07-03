#![no_std]

use spirv_std::glam::{vec4, Vec2, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn fs_main(#[spirv(frag_coord)] coord: Vec4, output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0); // Outputs red color
}

#[spirv(vertex)]
pub fn vs_main(#[spirv(vertex_index)] idx: u32, #[spirv(position, invariant)] out_pos: &mut Vec4) {
    // generate full-screen triangle
    let x = if idx == 1 { 3.0 } else { -1.0 };
    let y = if idx == 2 { 3.0 } else { -1.0 };
    *out_pos = vec4(x, y, 0.0, 1.0);
}
