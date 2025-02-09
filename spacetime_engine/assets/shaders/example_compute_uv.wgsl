@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = vec2<f32>(textureDimensions(output_texture));
    let uv = vec2<f32>(global_id.xy) / dimensions;
    textureStore(output_texture, global_id.xy, vec4<f32>(uv, 0.5, 1.0));
}
