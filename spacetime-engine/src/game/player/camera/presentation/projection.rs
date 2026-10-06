//! Contextual USF camera mirrors the local view from a bounded semantic anchor.

use super::*;

/// Contextual USF projection uses a bounded presentation domain, not the active
/// physical Scale Slice. Do not inherit subject-metre clip conversion here.
const USF_PROJECTION_NEAR_UNITS: f32 = 0.001;
const USF_PROJECTION_FAR_UNITS: f32 = 100_000.0;

fn apply_usf_projection_clip_domain(projection: &mut Projection) {
    let Projection::Perspective(perspective) = projection else {
        return;
    };
    perspective.near = USF_PROJECTION_NEAR_UNITS;
    perspective.far = USF_PROJECTION_FAR_UNITS;
}

fn projection_eye_offset_metres(
    semantic_runtime_anchor: Vec3,
    camera_runtime_position: Vec3,
    interaction_scale: crate::spatial::SpatialScale,
) -> Option<DVec3> {
    let offset_native = camera_runtime_position - semantic_runtime_anchor;
    if !offset_native.is_finite() {
        return None;
    }
    let metres_per_native = interaction_scale.metres_per_native();
    if !metres_per_native.is_finite() || metres_per_native <= 0.0 {
        return None;
    }
    let offset = DVec3::new(
        f64::from(offset_native.x) * metres_per_native,
        f64::from(offset_native.y) * metres_per_native,
        f64::from(offset_native.z) * metres_per_native,
    );
    offset.is_finite().then_some(offset)
}

/// Mirrors local view orientation/projection into the dedicated USF pass while
/// keeping the projection camera itself on the bounded semantic runtime anchor.
///
/// Eye height / cockpit / third-person boom remain presentation-only, but they
/// are explicitly published into `UsfViewContext` in SI metres. Contextual
/// geometry subtracts that offset before view-chart scaling, preserving the
/// exact camera ray without turning camera-rig motion into semantic authority.
pub(in crate::game::player) fn sync_usf_projection_camera(
    freecam: Res<DebugFreecam>,
    observation: Res<crate::spatial::UsfViewObservationOverride>,
    target: Single<
        (&Transform, &UsfScaleLayer),
        (
            With<LocalViewTarget>,
            Without<PlayerCamera>,
            Without<UsfProjectionCamera>,
        ),
    >,
    local: Single<
        (
            &Transform,
            &Projection,
            &Camera,
            &bevy::camera::RenderTarget,
        ),
        (With<PlayerCamera>, Without<UsfProjectionCamera>),
    >,
    far: Single<
        (
            &mut Transform,
            &mut Projection,
            &mut Camera,
            &mut bevy::camera::RenderTarget,
            &mut crate::spatial::UsfViewContext,
        ),
        (With<UsfProjectionCamera>, Without<PlayerCamera>),
    >,
) {
    let (target, target_layer) = target.into_inner();
    let (local_transform, local_projection, local_camera, local_target) = local.into_inner();
    let (mut far_transform, mut far_projection, mut far_camera, mut far_target, mut view) =
        far.into_inner();

    far_camera.viewport = local_camera.viewport.clone();
    *far_target = local_target.clone();
    far_transform.scale = Vec3::ONE;

    if !freecam.enabled() {
        let eye_offset = projection_eye_offset_metres(
            target.translation,
            local_transform.translation,
            target_layer.scale(),
        )
        .unwrap_or(DVec3::ZERO);
        view.set_projection_eye_offset_metres(eye_offset);

        far_transform.translation = target.translation;
        far_transform.rotation = local_transform.rotation;
        *far_projection = local_projection.clone();
        apply_usf_projection_clip_domain(&mut far_projection);
        far_camera.is_active = local_camera.is_active;
        return;
    }

    // A freecam observation override already makes the camera eye itself the
    // semantic presentation anchor. There is no additional rig offset to apply.
    view.set_projection_eye_offset_metres(DVec3::ZERO);

    match freecam.projection_policy() {
        FreecamProjectionPolicy::Follow => {
            far_transform.translation = observation
                .current()
                .map_or(target.translation, |(_, runtime)| runtime);
            far_transform.rotation = local_transform.rotation;
            *far_projection = local_projection.clone();
            apply_usf_projection_clip_domain(&mut far_projection);
            far_camera.is_active = local_camera.is_active;
        }
        FreecamProjectionPolicy::Frozen => {
            if let Some((_, runtime)) = observation.current() {
                far_transform.translation = runtime;
            }
            far_camera.is_active = local_camera.is_active;
        }
        FreecamProjectionPolicy::Disabled => {
            far_camera.is_active = false;
        }
    }
}
