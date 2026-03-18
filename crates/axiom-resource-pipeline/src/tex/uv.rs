pub fn remap_uv(
    java_uv:   [f32; 4],
    src_w:     u32,
    src_h:     u32,
    dst_w:     u32,
    dst_h:     u32,
) -> [f32; 4] {
    let sx = dst_w as f32 / src_w as f32;
    let sy = dst_h as f32 / src_h as f32;
    [
        java_uv[0] * sx,
        java_uv[1] * sy,
        java_uv[2] * sx,
        java_uv[3] * sy,
    ]
}

pub fn uv_to_bedrock(uv: [f32; 4]) -> ([f32; 2], [f32; 2]) {
    let pos  = [uv[0], uv[1]];
    let size = [uv[2] - uv[0], uv[3] - uv[1]];
    (pos, size)
}

pub fn rotate_uv(uv: [f32; 4], rotation: i32) -> [f32; 4] {
    match rotation % 360 {
        90  => [uv[1], 16.0 - uv[2], uv[3], 16.0 - uv[0]],
        180 => [16.0 - uv[2], 16.0 - uv[3], 16.0 - uv[0], 16.0 - uv[1]],
        270 => [16.0 - uv[3], uv[0], 16.0 - uv[1], uv[2]],
        _   => uv,
    }
}
