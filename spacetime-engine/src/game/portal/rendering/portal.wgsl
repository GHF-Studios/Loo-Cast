#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var portal_texture: texture_2d<f32>;

@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var portal_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var<uniform> two_sided: u32;

@fragment
fn fragment(
    mesh: VertexOutput,
    @builtin(front_facing) front_facing: bool,
) -> @location(0) vec4<f32> {
    // GPU backface culling is deliberately disabled. One-sidedness belongs to
    // the portal mechanic rather than to polygon winding.
    if !front_facing && two_sided == 0u {
        discard;
    }

    let uv = coords_to_viewport_uv(
        mesh.position.xy,
        view.viewport,
    );

    return textureSample(
        portal_texture,
        portal_sampler,
        uv,
    );
}