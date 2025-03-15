struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) depth: f32
}

@group(0) @binding(0)
var<uniform> camera: mat4x4f;

@vertex
fn v_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera * vec4f(in.pos, 1.0);
    out.depth = in.pos.z;
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4f {
    let clamped = clamp(1.0 / abs(in.depth), 0.0, 1.0);
    return vec4f(clamped, clamped, clamped, 1.0);
}