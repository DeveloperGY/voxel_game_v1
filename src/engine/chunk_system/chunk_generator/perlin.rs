pub fn perlin(v_x: i32, v_y: i32) -> i32 {
    let x = v_x as f32 / 16.0;
    let y = v_y as f32 / 16.0;

    ((x.sin() * y.sin() + 1.0) * 8.0).trunc() as i32
}