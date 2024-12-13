struct AppState {
    pos_x: f32,
    pos_y: f32,
    zoom: f32,
    // TODO: Add the diffrerent gravitons and their data
}

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> app_state: AppState;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // Full-screen triangle
    var vertices = array<vec2<f32>, 3>(
        vec2<f32>(-1., 1.),
        vec2<f32>(3.0, 1.),
        vec2<f32>(-1., -3.0),
    );

    var out: VertexOutput;
    out.coord = vertices[in.vertex_index];
    out.position = vec4<f32>(out.coord, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(255, 0, 0, 1.0); // Blue is constant, alpha is 1.0
}
