#![no_std]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn fs_main(output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0); // Outputs red color
}
