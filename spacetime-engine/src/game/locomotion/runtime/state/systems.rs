//! Apply locomotion policy to the controlled ECS subject and its runtime body.

use super::policy::*;
use super::*;

pub(in crate::game::locomotion) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &TravelState,
            &TravelProfile,
            &LocomotionCapabilities,
            &LocomotionEnabled,
            &LocomotionInhibition,
            Option<&LocomotionRegimeOverride>,
            &mut ControlledSubjectLocomotion,
            &mut UsfCanonicalMotion,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        detailed,
        travel,
        profile,
        capabilities,
        enabled,
        inhibition,
        regime_override,
        mut locomotion,
        mut motion,
    ) = subject.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = locomotion.kernel();

    if !enabled.0 || inhibition.is_inhibited() {
        let collision_policy = locomotion.collision_policy();
        if locomotion.resolve(
            previous_regime,
            MotionKernel::Disabled,
            collision_policy,
            VelocitySemantics::Zero,
        ) {
            transitions.write(ControlledSubjectLocomotionChanged {
                entity,
                previous_regime,
                regime: locomotion.regime(),
                previous_kernel,
                kernel: locomotion.kernel(),
            });
        }
        motion.set_canonical_authority(false);
        return;
    }

    let regime = select_regime(
        RegimeSelection {
            entity,
            previous_regime,
            layer,
            detailed,
            travel,
            profile,
            capabilities,
            regime_override,
        },
        &mut locomotion,
    );

    let (kernel, collision_policy, velocity_semantics) = motion_contract(
        regime,
        layer.scale(),
        detailed.0,
        *capabilities,
        locomotion.thrusters_enabled(),
    );

    let changed = locomotion.resolve(regime, kernel, collision_policy, velocity_semantics);
    motion.set_canonical_authority(canonical_motion_authoritative(
        kernel,
        layer.scale(),
        detailed.0,
    ));

    if changed {
        transitions.write(ControlledSubjectLocomotionChanged {
            entity,
            previous_regime,
            regime: locomotion.regime(),
            previous_kernel,
            kernel: locomotion.kernel(),
        });
    }
}

pub(in crate::game::locomotion) fn sync_locomotion_runtime(
    mut commands: Commands,
    subject: Single<
        (
            Entity,
            Ref<UsfScaleLayer>,
            &PhysicalBoxHull,
            Option<&ScaleInteractionProxy>,
            &ControlledSubjectLocomotion,
            &LocomotionEnabled,
            Option<&CharacterMotor>,
            Option<&Collider>,
            Option<&DetailedBodyCollision>,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        hull,
        proxy,
        locomotion,
        enabled,
        motor,
        collider,
        detailed_collision,
        mut input,
        mut ground,
    ) = subject.into_inner();

    if !enabled.0 {
        if motor.is_some() {
            commands.entity(entity).remove::<CharacterMotor>();
        }
        return;
    }

    if layer.is_changed() {
        input.clear();
        ground.clear_for_rechart();
    }

    match locomotion.collision_policy() {
        CollisionPolicy::Disabled => {
            if collider.is_some() || detailed_collision.is_some() {
                commands
                    .entity(entity)
                    .remove::<Collider>()
                    .remove::<DetailedBodyCollision>();
            }
        }
        CollisionPolicy::DetailedBody => {
            if collider.is_none() || layer.is_changed() || detailed_collision.is_none() {
                commands
                    .entity(entity)
                    .insert((hull.collider(layer.scale()), DetailedBodyCollision));
            }
        }
        CollisionPolicy::ScaleProxy => {
            if collider.is_none() || layer.is_changed() || detailed_collision.is_some() {
                let clearance_metres = proxy.map_or(0.0, |proxy| proxy.clearance_metres());
                commands
                    .entity(entity)
                    .insert(hull.bounding_sphere_collider(layer.scale(), clearance_metres))
                    .remove::<DetailedBodyCollision>();
            }
        }
    }

    let wants_character_motor = locomotion.kernel() == MotionKernel::Character;
    if wants_character_motor && motor.is_none() {
        commands.entity(entity).insert(CharacterMotor);
    } else if !wants_character_motor && motor.is_some() {
        commands.entity(entity).remove::<CharacterMotor>();
    }
}
