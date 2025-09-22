struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: u32,
    chunk_scale: u32,
    current_view_scale: u32,
    _padding: vec3<u32>,
};

@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: ShaderParams;

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> vec3<f32> {
    let i = floor(h * 6.0);
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let mod_i = i % 6.0;

    return select(
        select(
            select(
                select(
                    select(
                        vec3(v, t, p),
                        vec3(q, v, p), mod_i == 1.0),
                    vec3(p, v, t), mod_i == 2.0),
                vec3(p, q, v), mod_i == 3.0),
            vec3(t, p, v), mod_i == 4.0),
        vec3(v, p, q), mod_i == 5.0);
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let normalized_scale = clamp((f32(params.chunk_scale) + 35.0) / 70.0, 0.0, 1.0);
    let base_color = hsv_to_rgb(normalized_scale, 1.0, 1.0);

    let border_thickness = 16u;
    let is_border_x = global_id.x < border_thickness || global_id.x >= (params.chunk_size - border_thickness);
    let is_border_y = global_id.y < border_thickness || global_id.y >= (params.chunk_size - border_thickness);
    let is_border = is_border_x || is_border_y;

    var color: vec4<f32>;
    if (is_border) {
        color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    } else {
        color = vec4<f32>(base_color, 0.05);
    }

    textureStore(output_texture, global_id.xy, color);
}
