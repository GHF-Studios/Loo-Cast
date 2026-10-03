#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<storage, read> refinement_clip_boxes: array<vec4<f32>>;

@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var<uniform> refinement_clip_meta: vec4<u32>;

// analytical-procedural-debug-grid-v1
// x = enabled, y = physical metres per incoming UV unit.
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var<uniform> debug_grid_meta: vec4<f32>;

fn aa_grid_line(metric: vec2<f32>, step_metres: f32, width_pixels: f32) -> f32 {
    let q = metric / max(step_metres, 1.0e-6);
    let phase = fract(q);
    let distance_to_line = min(phase, vec2<f32>(1.0) - phase);
    let derivative = max(fwidth(q), vec2<f32>(1.0e-5));
    let edge0 = derivative * 0.20;
    let edge1 = derivative * max(width_pixels, 0.25);
    let line = vec2<f32>(1.0) - smoothstep(edge0, edge1, distance_to_line);
    return max(line.x, line.y);
}

fn analytical_debug_grid(uv: vec2<f32>) -> vec3<f32> {
    let metric = uv * debug_grid_meta.y;
    let footprint = max(
        max(fwidth(metric).x, fwidth(metric).y),
        1.0e-5,
    );

    // Choose a binary metric grid level whose minor lines remain several
    // pixels apart. The grid is one function over all scales: coarser levels
    // are aligned supersets of finer ones rather than separately generated
    // textures, and fwidth gives analytical anti-aliasing.
    let target_minor_step = max(footprint * 8.0, 0.25);
    let minor_step = exp2(ceil(log2(target_minor_step)));
    let major_step = minor_step * 8.0;

    let minor = aa_grid_line(metric, minor_step, 0.90);
    let major = aa_grid_line(metric, major_step, 1.35);

    let fill = vec3<f32>(0.74, 0.76, 0.79);
    let minor_color = vec3<f32>(0.23, 0.25, 0.28);
    let major_color = vec3<f32>(0.96, 0.50, 0.12);

    var color = mix(fill, minor_color, minor * 0.78);
    color = mix(color, major_color, major * 0.94);
    return color;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    let clip_count = refinement_clip_meta.x;
    var clip_index = 0u;

    loop {
        if clip_index >= clip_count {
            break;
        }

        let base = clip_index * 2u;
        let center_data = refinement_clip_boxes[base];
        let half_data = refinement_clip_boxes[base + 1u];
        let center = center_data.xyz;
        let half_extent = half_data.xyz;
        let exposed_faces = u32(center_data.w + 0.5);
        let support_band = max(half_data.w, 0.0);
        let local = in.world_position.xyz - center;

        var clip_min = -half_extent;
        var clip_max = half_extent;

        // Fine realized coverage owns the aperture interior. On an exposed
        // parent/child frontier face, retain one narrow coarse support band
        // *inside* the fine aperture instead of cutting the parent exactly at
        // the child's AABB wall. Fine/fine internal faces keep a hard clip.
        if (exposed_faces & 1u) != 0u {
            clip_min.x += support_band;
        }
        if (exposed_faces & 2u) != 0u {
            clip_max.x -= support_band;
        }
        if (exposed_faces & 4u) != 0u {
            clip_min.y += support_band;
        }
        if (exposed_faces & 8u) != 0u {
            clip_max.y -= support_band;
        }
        if (exposed_faces & 16u) != 0u {
            clip_min.z += support_band;
        }
        if (exposed_faces & 32u) != 0u {
            clip_max.z -= support_band;
        }

        if all(clip_min <= clip_max)
            && all(local >= clip_min)
            && all(local <= clip_max)
        {
            discard;
        }

        clip_index += 1u;
    }

    var pbr_input = pbr_input_from_standard_material(in, is_front);

    if debug_grid_meta.x > 0.5 {
        pbr_input.material.base_color *= vec4<f32>(
            analytical_debug_grid(in.uv),
            1.0,
        );
    }

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
