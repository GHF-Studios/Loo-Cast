@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<storage, read> params: array<f32>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = vec2<f32>(textureDimensions(output_texture));
    let uv = vec2<f32>(global_id.xy) / dimensions;

    // Define shrink ranges for each axis
    let shrink = 0.1;
    let min_x = -shrink;
    let max_x = 1 + shrink;
    let min_y = -shrink;
    let max_y = 1 + shrink;

    // Apply shrink transformation
    let shrunk_uv = vec2<f32>(
        min_x + uv.x * (max_x - min_x),
        min_y + uv.y * (max_y - min_y)
    );

    textureStore(output_texture, global_id.xy, vec4<f32>(shrunk_uv, 0.0, 1.0));
}
