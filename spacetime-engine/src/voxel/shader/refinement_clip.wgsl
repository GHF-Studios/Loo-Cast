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
