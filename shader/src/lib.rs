#![no_std]

use bytemuck::{Pod, Zeroable};
use spirv_std::{
    glam::{vec2, vec4, UVec3, Vec2, Vec3Swizzles, Vec4, Vec4Swizzles},
    image::StorageImage2d,
    num_traits::Float,
    spirv, Image, Sampler,
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

#[spirv(compute(threads(16, 16)))]
pub fn cs_main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] data: &Data,
    #[spirv(descriptor_set = 0, binding = 1)] output: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
) {
    const LIMIT: f32 = 100. * 100.;
    // find where the pixel falls into
    let mut coord = id.as_vec3().xy();
    let mut velocity = Vec2::ZERO;
    for _ in 0..1000 {
        let mut min_distance_squared = f32::MAX;
        for i in 0..data.num_gravitons {
            let graviton = data.gravitons[i as usize];
            let graviton_pos = vec2(graviton.position_x, graviton.position_y);

            // calculate gravity
            let distance_squared = graviton_pos.distance_squared(coord);
            min_distance_squared = min_distance_squared.min(distance_squared);
            // check if inside graviton
            if distance_squared < LIMIT {
                // no documentation for why this is unsafe (probably because of mutation through shared reference?)
                unsafe {
                    // return color
                    output.write(
                        id.truncate(),
                        vec4(graviton.color_r, graviton.color_g, graviton.color_b, 1.),
                    );
                }
                return;
            }
        }
        // make bigger steps if far from any gravitons
        let dt = (min_distance_squared.sqrt() * 0.1).clamp(0.002, 0.05);
        let [new_coord, new_velocity] = rk4_step(coord, velocity, dt, data);
        coord = new_coord;
        velocity = new_velocity;
    }
}

/// Compute total gravitational acceleration
fn accel(coord: Vec2, data: &Data) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    for i in 0..data.num_gravitons {
        let graviton = data.gravitons[i as usize];
        let graviton_pos = vec2(graviton.position_x, graviton.position_y);
        let vector = graviton_pos - coord;

        let distance_squared = graviton_pos.distance_squared(coord);
        let distance = distance_squared.sqrt();
        acceleration += vector * (graviton.mass / (distance_squared * distance));
    }
    acceleration
}

fn rk4_step(coord: Vec2, velocity: Vec2, dt: f32, data: &Data) -> [Vec2; 2] {
    // k1
    let a1 = accel(coord, data);
    let p1 = velocity;

    // k2
    let pos2 = coord + p1 * (dt * 0.5);
    let vel2 = velocity + a1 * (dt * 0.5);
    let a2 = accel(pos2, data);
    let p2 = vel2;

    // k3
    let pos3 = coord + p2 * (dt * 0.5);
    let vel3 = velocity + a2 * (dt * 0.5);
    let a3 = accel(pos3, data);
    let p3 = vel3;

    // k4
    let pos4 = coord + p3 * dt;
    let vel4 = velocity + a3 * dt;
    let a4 = accel(pos4, data);
    let p4 = vel4;

    // Combine increments
    let pos_inc = (p1 + p2 * 2.0 + p3 * 2.0 + p4) * (dt / 6.0);
    let vel_inc = (a1 + a2 * 2.0 + a3 * 2.0 + a4) * (dt / 6.0);

    [coord + pos_inc, velocity + vel_inc]
}

#[spirv(vertex)]
pub fn vs_main(#[spirv(vertex_index)] idx: u32, #[spirv(position, invariant)] out_pos: &mut Vec4) {
    // generate full-screen triangle
    let x = if idx == 1 { 3. } else { -1. };
    let y = if idx == 2 { 3. } else { -1. };
    *out_pos = vec4(x, y, 0., 1.);
}

#[spirv(fragment)]
pub fn fs_main(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(descriptor_set = 0, binding = 0)] texture: &Image!(2D, format = rgba32f, sampled),
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    out_color: &mut Vec4,
) {
    *out_color = texture.sample(*sampler, coord.xy());
}
