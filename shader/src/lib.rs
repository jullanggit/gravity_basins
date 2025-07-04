#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec4, Vec2, Vec4};
use spirv_std::spirv;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Data {
    pub gravitons: [Graviton; 32],
    pub num_gravitons: u32,
}
impl Data {
    pub fn new(gravitons: [Graviton; 32], num_gravitons: u32) -> Self {
        Self {
            gravitons,
            num_gravitons,
        }
    }
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Graviton {
    pub position: [f32; 2],
    pub color: [f32; 3],
}
impl Graviton {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] data: &Data,
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
