#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::{
    glam::{vec2, vec4, Vec2, Vec4, Vec4Swizzles},
    num_traits::Float,
    spirv,
};

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
    pub mass: f32,
    _pad1: f32,
}
impl Graviton {
    pub fn new(
        position_x: f32,
        position_y: f32,
        color_r: f32,
        color_g: f32,
        color_b: f32,
        mass: f32,
    ) -> Self {
        Self {
            color_r,
            color_g,
            color_b,
            position_x,
            position_y,
            mass,
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
    const LIMIT: f32 = 100. * 100.;
    const DT: f32 = 0.01;
    // find where the pixel falls into
    let mut coord = coord.xy();
    let mut velocity = Vec2::ZERO;
    for _ in 0..1000 {
        let mut delta_velocity = Vec2::ZERO;
        for i in 0..data.num_gravitons {
            let graviton = data.gravitons[i as usize];
            let graviton_pos = vec2(graviton.position_x, graviton.position_y);

            // calculate gravity
            let distance_squared = graviton_pos.distance_squared(coord);
            // check if inside graviton
            if distance_squared < LIMIT {
                // return color
                *output = vec4(graviton.color_r, graviton.color_g, graviton.color_b, 1.);
                return;
            }
            let vector = graviton_pos - coord;
            delta_velocity +=
                vector * (graviton.mass / (distance_squared * distance_squared.sqrt()));
        }
        velocity += delta_velocity * DT;
        coord += velocity * DT;
    }
}

#[spirv(vertex)]
pub fn vs_main(#[spirv(vertex_index)] idx: u32, #[spirv(position, invariant)] out_pos: &mut Vec4) {
    // generate full-screen triangle
    let x = if idx == 1 { 3. } else { -1. };
    let y = if idx == 2 { 3. } else { -1. };
    *out_pos = vec4(x, y, 0., 1.);
}
