#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec2, vec4, Vec4};
use spirv_std::spirv;

#[derive(Pod, Zeroable, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Data {
    pub gravitons: [Graviton; 32],
    pub num_gravitons: u32,
    /// pad to 16 bytes, see `Graviton` for details
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
}
impl Data {
    pub fn new(gravitons: [Graviton; 32], num_gravitons: u32) -> Self {
        Self {
            gravitons,
            num_gravitons,
            ..Default::default()
        }
    }
}

#[derive(Pod, Zeroable, Clone, Copy, Default)]
#[repr(C, align(16))]
pub struct Graviton {
    // color is a vec3<f32> in wgpu, which has an alignment of 16 bytes, so we need to align the entire struct to that
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    _pad0: f32,
    pub position_x: f32,
    pub position_y: f32,
    _pad1: f32,
    _pad2: f32,
}
impl Graviton {
    pub fn new(position_x: f32, position_y: f32, color_r: f32, color_g: f32, color_b: f32) -> Self {
        Self {
            color_r,
            color_g,
            color_b,
            position_x,
            position_y,
            ..Default::default()
        }
    }
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] data: &Data,
    output: &mut Vec4,
) {
    // find closest graviton
    let coord = coord.truncate().truncate();
    let mut closest_index = 0;
    let mut distance = f32::MAX;
    for i in 0..data.num_gravitons {
        let graviton = data.gravitons[i as usize];
        let new_distance = vec2(graviton.position_x, graviton.position_y).distance(coord);
        if new_distance < distance {
            distance = new_distance;
            closest_index = i;
        }
    }
    let closest_graviton = data.gravitons[closest_index as usize];
    *output = vec4(
        closest_graviton.color_r,
        closest_graviton.color_g,
        closest_graviton.color_b,
        1.,
    );
}

#[spirv(vertex)]
pub fn vs_main(#[spirv(vertex_index)] idx: u32, #[spirv(position, invariant)] out_pos: &mut Vec4) {
    // generate full-screen triangle
    let x = if idx == 1 { 3. } else { -1. };
    let y = if idx == 2 { 3. } else { -1. };
    *out_pos = vec4(x, y, 0., 1.);
}
