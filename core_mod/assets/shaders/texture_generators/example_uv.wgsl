struct ShaderParams {
    chunk_pos: vec2<i32>,
    chunk_size: u32,
    chunk_scale: i32,
    _current_view_scale: i32,
    zoom_factor: f32,
    _padding: vec2<u32>,
};

@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: ShaderParams;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x >= 1000 || global_id.y >= 1000) {
        return;
    }
    
    let tex_size = vec2<f32>(textureDimensions(output_texture));
    let uv = vec2<f32>(global_id.xy) / tex_size;
    let flipped_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    let color = vec4<f32>(uv.x, uv.y, 0.0, 1.0);
    textureStore(output_texture, global_id.xy, color);
}