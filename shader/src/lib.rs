#![no_std]

use spirv_std::glam::{vec4, Vec2, Vec4};
use spirv_std::spirv;

pub struct Data {
    pub gravitons: [Option<Graviton>; 32],
}
impl Data {
    pub fn new(gravitons: [Option<Graviton>; 32]) -> Self {
        Self { gravitons }
    }
}

pub struct Graviton {
    pub position: Vec2,
    pub color: Vec4,
}
impl Graviton {
    pub fn new(position: Vec2, color: Vec4) -> Self {
        Self { position, color }
    }
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(push_constant)] data: &Data,
    output: &mut Vec4,
) {
    *output = if coord.x < 400. {
        vec4(1., 0., 0., 1.)
    } else {
        vec4(0., 1., 0., 1.)
    };
}

#[spirv(vertex)]
pub fn vs_main(#[spirv(vertex_index)] idx: u32, #[spirv(position, invariant)] out_pos: &mut Vec4) {
    // generate full-screen triangle
    let x = if idx == 1 { 3. } else { -1. };
    let y = if idx == 2 { 3. } else { -1. };
    *out_pos = vec4(x, y, 0., 1.);
}
