struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) normal: vec3f
}

@group(0) @binding(0)
var<uniform> camera: mat4x4f;

@vertex
fn v_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera * vec4f(in.pos, 1.0);
    out.normal = in.normal;
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4f {
//    let inverse_depth = 1.0 / pow(abs(in.v_pos.z), 2.0);
//    return vec4f(inverse_depth, 0.0, inverse_depth, 1.0);
//    return vec4f(1.0, 0.0, 1.0, 1.0);
    return vec4f(abs(in.normal) * 0.25, 1.0);
}