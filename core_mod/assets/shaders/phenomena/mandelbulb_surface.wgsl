#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
}

struct PhenomenonSurfaceParams {
    primary: vec4<f32>,
    secondary: vec4<f32>,
    glow: vec4<f32>,
    // x=layer_norm, y=window_scale, z=time_seconds, w=emissive_strength
    meta: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: PhenomenonSurfaceParams;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var metric_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var metric_sampler: sampler;

fn saturate(value: f32) -> f32 {
    return clamp(value, 0.0, 1.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = vec2<f32>(0.5, 0.5);
#ifdef VERTEX_UVS_A
    uv = in.uv;
#endif

    let metric_rgb = textureSample(metric_texture, metric_sampler, uv).rgb;
    let metric = dot(metric_rgb, vec3<f32>(0.299, 0.587, 0.114));
    let layer_norm = saturate(material.meta.x);
    let window_scale = saturate(material.meta.y);
    let time_seconds = material.meta.z;
    let emissive_strength = material.meta.w;

    let normal = normalize(in.world_normal);
    let light_dir = normalize(vec3<f32>(0.42, 0.83, 0.37));
    let ndotl = saturate(dot(normal, light_dir));
    let view_dir = normalize(view.world_position - in.world_position.xyz);
    let rim = pow(1.0 - saturate(dot(normal, view_dir)), 2.0);

    let base_t = saturate(metric * 0.75 + layer_norm * 0.25);
    var albedo = mix(material.primary.rgb, material.secondary.rgb, base_t);

    let band_phase = in.world_position.z * 0.03 + time_seconds * 1.2 + metric * 16.0;
    let bands = 0.5 + 0.5 * sin(band_phase);
    let glow_mix = bands * (1.0 - window_scale) * 0.55;
    albedo = mix(albedo, material.glow.rgb, glow_mix);

    let lit = albedo * (0.15 + ndotl * 0.85);
    let emissive = material.glow.rgb * (rim * emissive_strength * (0.5 + 0.5 * bands));
    let color = lit + emissive;

    return vec4<f32>(color, material.primary.a);
}
