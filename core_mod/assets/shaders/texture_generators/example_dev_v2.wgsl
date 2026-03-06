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

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let chunk_size = params.chunk_size;
    if (global_id.x >= chunk_size || global_id.y >= chunk_size) {
        return;
    }

    let subgrid_size = max(1u, chunk_size / 10u);
    let subgrid_size_i = i32(subgrid_size);
    let chunk_size_i = i32(chunk_size);

    let world_x = params.chunk_pos.x * chunk_size_i + i32(global_id.x);
    let world_y = params.chunk_pos.y * chunk_size_i + i32(global_id.y);
    let sub_x = positive_mod(world_x, subgrid_size_i);
    let sub_y = positive_mod(world_y, subgrid_size_i);

    let major_thickness = 6u;
    let minor_thickness = 2;

    let is_chunk_border = global_id.x < major_thickness
        || global_id.y < major_thickness
        || global_id.x >= (chunk_size - major_thickness)
        || global_id.y >= (chunk_size - major_thickness);

    let is_subgrid_line = sub_x < minor_thickness
        || sub_y < minor_thickness
        || sub_x >= (subgrid_size_i - minor_thickness)
        || sub_y >= (subgrid_size_i - minor_thickness);

    let parity = (params.chunk_pos.x + params.chunk_pos.y + params.chunk_scale) & 1;
    let is_current_scale = abs(params.chunk_scale - params.current_view_scale) == 0;

    let dark_base = vec3<f32>(0.08, 0.09, 0.10);
    let light_base = vec3<f32>(0.11, 0.12, 0.14);
    var base = select(dark_base, light_base, parity == 1);
    if (is_current_scale) {
        base *= 1.15;
    } else {
        base *= 0.90;
    }

    var color = vec3<f32>(base);
    if (is_subgrid_line) {
        color = vec3<f32>(0.40, 0.42, 0.45);
    }
    if (is_chunk_border) {
        color = vec3<f32>(0.96, 0.98, 1.00);
    }

    textureStore(output_texture, global_id.xy, vec4<f32>(color, 1.0));
}
