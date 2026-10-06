//! Temporary bounded audit for primary control/view/interaction coherence.
//!
//! Emits one warning only when the observed state tuple changes. It owns no
//! gameplay state and is safe to remove once the current regression is proven.

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    ecs::{UsfOwnershipQuery, UsfPresentationProjectionOf},
    game::{
        control::{LocalControlSubject, LocalViewTarget},
        locomotion::{DetailedBodyScale, MotionExecution},
        player::{CameraMode, PlayerCamera, ViewCameraProfile},
        surface::SurfaceContext,
    },
    spatial::{
        UsfInteractionScaleAffinity, UsfPosition, UsfPrimaryInteractionSlice,
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
    },
    view::ViewSubjectPresentation,
};

pub(super) fn audit_primary_runtime_coherence(
    interaction: Res<UsfPrimaryInteractionSlice>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    controlled: Query<
        (
            Entity,
            &UsfScaleLayer,
            &UsfInteractionScaleAffinity,
            &DetailedBodyScale,
            &MotionExecution,
            &SurfaceContext,
            &Visibility,
            Option<&ViewCameraProfile>,
        ),
        With<LocalControlSubject>,
    >,
    view_targets: Query<Entity, With<LocalViewTarget>>,
    cameras: Query<&PlayerCamera>,
    presentations: Query<
        (&UsfPresentationProjectionOf, &Visibility, Option<&RenderLayers>),
        With<ViewSubjectPresentation>,
    >,
    mut previous: Local<Option<String>>,
) {
    let controlled_count = controlled.iter().count();
    let view_target_count = view_targets.iter().count();
    let camera_count = cameras.iter().count();

    let snapshot = if let Ok((
        runtime,
        layer,
        affinity,
        detailed,
        execution,
        surface,
        parent_visibility,
        profile,
    )) = controlled.single()
    {
        let semantic = ownership.semantic_of(runtime);
        let position = semantic.and_then(|semantic| semantic_positions.get(semantic).ok());
        let view_target = view_targets.single().ok();
        let camera = cameras.single().ok();

        let body = surface.body();
        let collision_near = |scale| {
            body.zip(position).is_some_and(|(body, position)| {
                coverage.has_near_for_authority(
                    body,
                    scale,
                    position,
                    UsfScaleRoleMask::COLLISION,
                    0.0,
                )
            })
        };
        let collision_fact_count = |scale| {
            body.map_or(0, |body| {
                coverage
                    .iter()
                    .filter(|fact| {
                        fact.authority() == body
                            && fact.scale() == scale
                            && fact.roles().contains(UsfScaleRoleMask::COLLISION)
                    })
                    .count()
            })
        };

        let mut self_presentations = 0usize;
        let mut self_layer0 = 0usize;
        let mut self_inherited = 0usize;
        for (projection, visibility, layers) in &presentations {
            if projection.0 != runtime {
                continue;
            }
            self_presentations += 1;
            if layers.is_some_and(|layers| *layers == RenderLayers::default()) {
                self_layer0 += 1;
            }
            if *visibility == Visibility::Inherited {
                self_inherited += 1;
            }
        }

        let camera_mode = camera.map(|camera| camera.mode);
        let third_person_resolved_metres = profile
            .map(|profile| profile.third_person.resolved_distance_metres)
            .unwrap_or(f32::NAN);

        let diagnosis = if view_target != Some(runtime) {
            "CONTROL_VIEW_DIVERGED"
        } else if layer.scale() != affinity.scale() {
            if collision_near(affinity.scale()) {
                "TARGET_COLLISION_PRESENT_HANDOFF_NOT_COMMITTED"
            } else {
                "WAITING_TARGET_COLLISION_COVERAGE"
            }
        } else if !surface.collision_ready() {
            "DETAILED_CHART_WITHOUT_LOCAL_COLLISION_COVERAGE"
        } else if matches!(camera_mode, Some(CameraMode::ThirdPerson)) && self_layer0 == 0 {
            "THIRD_PERSON_SELF_PRESENTATION_NOT_PROMOTED"
        } else {
            "COHERENT_OR_DOWNSTREAM"
        };

        format!(
            "RUNTIME_COHERENCE_AUDIT diagnosis={diagnosis} \
controlled={runtime:?} semantic={semantic:?} \
control_count={controlled_count} view_target={view_target:?} view_target_count={view_target_count} \
runtime_scale={} interaction_current={} interaction_requested={:?} interaction_target={} \
affinity_scale={} detailed_scale={} \
kernel={:?} collision_policy={:?} \
surface_body={body:?} surface_collision_ready={} \
current_collision_near={} current_collision_facts={} \
target_collision_near={} target_collision_facts={} \
camera_count={camera_count} camera_mode={camera_mode:?} third_person_resolved_metres={third_person_resolved_metres:.3} \
parent_visibility={parent_visibility:?} self_presentations={self_presentations} \
self_layer0={self_layer0} self_inherited={self_inherited}",
            layer.scale(),
            interaction.scale(),
            interaction.requested_scale(),
            interaction.target_scale(),
            affinity.scale(),
            detailed.0,
            execution.kernel(),
            execution.collision_policy(),
            surface.collision_ready(),
            collision_near(layer.scale()),
            collision_fact_count(layer.scale()),
            collision_near(affinity.scale()),
            collision_fact_count(affinity.scale()),
        )
    } else {
        format!(
            "RUNTIME_COHERENCE_AUDIT diagnosis=CONTROL_CARDINALITY_INVALID \
control_count={controlled_count} view_target_count={view_target_count} camera_count={camera_count}"
        )
    };

    if previous.as_ref() != Some(&snapshot) {
        warn!("{snapshot}");
        *previous = Some(snapshot);
    }
}
