@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Define a full-screen triangle
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), // Bottom-left
        vec2<f32>(3.0, -1.0),  // Bottom-right (out of bounds to form a triangle)
        vec2<f32>(-1.0, 3.0)   // Top-left (out of bounds to form a triangle)
    );
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    // Map the fragment coordinates to the range [0, 1]
    let u_position = frag_coord.xy / vec2<f32>(640.0, 480.0); // Assuming a 640x480 resolution

    // Color based on position: red for x, green for y
    return vec4<f32>(u_position.x, u_position.y, 0.5, 1.0); // Blue is constant, alpha is 1.0
}
