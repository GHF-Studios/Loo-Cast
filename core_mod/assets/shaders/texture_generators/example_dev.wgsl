struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: u32,
    chunk_scale: i32,
    current_view_scale: i32,
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
    if (global_id.x >= 1000 || global_id.y >= 1000) {
        return;
    }

    let normalized_scale = clamp((f32(params.chunk_scale) + 35.0) / 70.0, 0.0, 1.0);
    let base_color = hsv_to_rgb(normalized_scale, 1.0, 1.0);

    let checker = (params.chunk_pos.x + params.chunk_pos.y + params.chunk_scale) % 2;
    let opacity_multiplier = select(1.0, 4.0, checker == 1);

    var final_color = vec4<f32>(base_color, 0.05 * opacity_multiplier);
    
    // === Border Levels ===
    let divisions = array<u32, 5u>(1u, 2u, 8u, 32u, 128u);
    let thicknesses = array<u32, 5u>(16u, 8u, 4u, 2u, 1u);
    let opacities = array<f32, 5u>(1.0, 0.5, 0.25, 0.125, 0.0625);

    for (var i = 0u; i < 5u; i = i + 1u) {
        let div = divisions[i];
        let section_size = 1000 / div;
        let thickness = thicknesses[i];
        let opacity = opacities[i];

        let section_x = global_id.x % section_size;
        let section_y = global_id.y % section_size;

        let is_border_x = section_x < thickness || section_x >= (section_size - thickness);
        let is_border_y = section_y < thickness || section_y >= (section_size - thickness);
        let is_border = is_border_x || is_border_y;

        if (is_border) {
            final_color += vec4<f32>(1.0, 1.0, 1.0, opacity);
        }
    }

    final_color = clamp(final_color, vec4(0.0), vec4(1.0));
    textureStore(output_texture, global_id.xy, final_color);
}
