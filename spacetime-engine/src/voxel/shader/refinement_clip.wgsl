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
        let center = refinement_clip_boxes[base].xyz;
        let half_extent = refinement_clip_boxes[base + 1u].xyz;
        let delta = abs(in.world_position.xyz - center);

        // Fine realized coverage owns this bounded aperture. Discarding the
        // coarse fragment is the presentation-side make-before-break handoff.
        if all(delta <= half_extent) {
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
