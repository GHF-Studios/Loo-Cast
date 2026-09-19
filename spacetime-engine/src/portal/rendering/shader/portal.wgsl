#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var portal_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var portal_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // The portal texture represents an entire virtual camera viewport. Sampling
    // by screen position makes the mesh act as an aperture/mask rather than a
    // UV-mapped television screen.
    let uv =
        coords_to_viewport_uv(
            mesh.position.xy,
            view.viewport,
        );

    return textureSample(
        portal_texture,
        portal_sampler,
        uv,
    );
}
