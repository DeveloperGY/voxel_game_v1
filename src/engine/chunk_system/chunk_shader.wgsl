struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) normal: vec3f,
    @location(2) tex_coords: vec2f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) normal: vec3f,
    @location(1) tex_coords: vec2f
}

@group(0) @binding(0)
var<uniform> camera: mat4x4f;

@group(1) @binding(0)
var texture_atlas: texture_2d<f32>;

@group(1) @binding(1)
var texture_atlas_sampler: sampler;

@vertex
fn v_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = camera * vec4f(in.pos, 1.0);
    out.normal = in.normal;
    out.tex_coords = in.tex_coords;
    return out;
}

@fragment
fn f_main(in: VertexOutput) -> @location(0) vec4f {
//    let inverse_depth = 1.0 / pow(abs(in.v_pos.z), 2.0);
//    return vec4f(inverse_depth, 0.0, inverse_depth, 1.0);
//    return vec4f(1.0, 0.0, 1.0, 1.0);
    return textureSample(texture_atlas, texture_atlas_sampler, in.tex_coords);
}