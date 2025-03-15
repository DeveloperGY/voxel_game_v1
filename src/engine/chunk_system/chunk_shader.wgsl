struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
}

@group(0) @binding(0)
var<uniform> camera: mat4x4f;

@vertex
fn v_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera * vec4f(in.pos, 1.0);
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 1.0, 1.0);
}