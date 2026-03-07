struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: u32,
    chunk_scale: i32,
    current_view_scale: i32,
    _padding: vec3<u32>,
};

@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: ShaderParams;

fn positive_mod(value: i32, modulo: i32) -> i32 {
    let r = value % modulo;
    return select(r + modulo, r, r >= 0);
}

fn pow10_i32(exp: i32) -> i32 {
    var result = 1;
    for (var i = 0; i < exp; i = i + 1) {
        result *= 10;
    }
    return result;
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let chunk_size = params.chunk_size;
    if (global_id.x >= chunk_size || global_id.y >= chunk_size) {
        return;
    }

    let chunk_size_i = i32(chunk_size);
    let subgrid_size_i = max(1, chunk_size_i / 10);
    let scale_delta = max(0, params.chunk_scale - params.current_view_scale);
    let scale_factor = pow10_i32(scale_delta);

    let world_x = params.chunk_pos.x * chunk_size_i + i32(global_id.x) * scale_factor;
    let world_y = params.chunk_pos.y * chunk_size_i + i32(global_id.y) * scale_factor;

    let chunk_mod_x = positive_mod(world_x, chunk_size_i);
    let chunk_mod_y = positive_mod(world_y, chunk_size_i);
    let sub_x = positive_mod(world_x, subgrid_size_i);
    let sub_y = positive_mod(world_y, subgrid_size_i);

    let major_thickness = min(chunk_size_i / 2, max(4, scale_factor));
    let minor_thickness = min(subgrid_size_i / 2, max(2, scale_factor));

    let is_chunk_border = chunk_mod_x < major_thickness
        || chunk_mod_y < major_thickness
        || chunk_mod_x >= (chunk_size_i - major_thickness)
        || chunk_mod_y >= (chunk_size_i - major_thickness);

    let is_subgrid_line = sub_x < minor_thickness
        || sub_y < minor_thickness
        || sub_x >= (subgrid_size_i - minor_thickness)
        || sub_y >= (subgrid_size_i - minor_thickness);

    let parity = (params.chunk_pos.x + params.chunk_pos.y + params.chunk_scale) & 1;

    let dark_base = vec3<f32>(0.08, 0.09, 0.10);
    let light_base = vec3<f32>(0.11, 0.12, 0.14);
    var base = select(dark_base, light_base, parity == 1);
    let normalized_scale = clamp((f32(params.chunk_scale) + 35.0) / 70.0, 0.0, 1.0);
    base *= 0.92 + normalized_scale * 0.08;

    var color = vec3<f32>(base);
    if (is_subgrid_line) {
        color = vec3<f32>(0.38, 0.40, 0.43);
    }
    if (is_chunk_border) {
        color = vec3<f32>(0.96, 0.98, 1.00);
    }

    textureStore(output_texture, global_id.xy, vec4<f32>(color, 1.0));
}
