@vertex
// Define a full-screen triangle
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), // Bottom-left
        vec2<f32>(3.0, -1.0),  // Bottom-right (out of bounds to form a triangle)
        vec2<f32>(-1.0, 3.0)   // Top-left (out of bounds to form a triangle)
    );
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}
