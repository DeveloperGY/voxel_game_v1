struct VertexInput {
    @location(0) pos: vec3f
}

@vertex
fn v_main(in: VertexInput) -> @builtin(position) vec4f {
    return vec4f(in.pos, 1.0);
}

@fragment
fn f_main() -> @location(0) vec4f {
    return vec4f(1.0, 1.0, 1.0, 1.0);
}