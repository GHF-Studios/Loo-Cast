//! Apply locomotion policy to the controlled ECS subject and its runtime body.

use super::policy::*;
use super::*;

pub(in crate::game::locomotion) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    handoff_inputs: Query<(&Transform, &FlightControlIntent)>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &TravelState,
            &LocomotionCapabilities,
            &LocomotionEnabled,
            &LocomotionInhibition,
            Option<&DeveloperMotionOverride>,
            &TravelAssistanceState,
            Option<&LocomotionRegimeOverride>,
            &mut ControlledSubjectLocomotion,
            &mut MotionExecution,
            &mut UsfCanonicalMotion,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        detailed,
        travel,
        capabilities,
        enabled,
        inhibition,
        developer_motion,
        assistance,
        regime_override,
        mut locomotion,
        mut execution,
        mut motion,
        mut runtime_velocity,
    ) = subject.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = execution.kernel();
    let previous_collision = execution.collision_policy();
    let previous_authority = motion.authority();
    let (body, intent) = handoff_inputs
        .get(entity)
        .expect("controlled motion handoff inputs");
    let before = MotionHandoffSnapshot {
        position: ownership
            .semantic_of(entity)
            .and_then(|semantic| semantic_positions.get(semantic).ok())
            .copied(),
        velocity_metres_per_second: motion.velocity_metres_per_second(),
        angular_velocity_radians_per_second: motion.angular_velocity_radians_per_second(),
        orientation: body.rotation,
        control_intent: *intent,
    };

    let decision = decide_motion_policy(
        MotionPolicyInputs {
            entity,
            layer: layer.scale(),
            detailed: detailed.0,
            capabilities: *capabilities,
            enabled: enabled.0,
            inhibited: inhibition.is_inhibited(),
            cruise_active: assistance.mode() == TravelAssistance::Cruise,
            regime_override,
            developer_motion,
            current_collision: execution.collision_policy(),
        },
        &mut locomotion,
    );
    match decision.velocity {
        VelocitySemantics::Zero => {
            motion.stop();
            motion.set_angular_velocity_radians_per_second(bevy::math::DVec3::ZERO);
            runtime_velocity.0 = Vec3::ZERO;
        }
        VelocitySemantics::PreserveCanonical => {
            if !motion.is_canonical_kinematic()
                && decision.authority == UsfMotionAuthority::CanonicalKinematics
            {
                motion.set_from_native_velocity(layer.scale(), runtime_velocity.0);
            } else if motion.is_canonical_kinematic()
                && decision.authority == UsfMotionAuthority::RuntimePhysics
            {
                runtime_velocity.0 = motion.native_velocity(layer.scale());
            }
        }
    }
    let regime_changed = locomotion.resolve(decision.regime);
    let execution_changed = execution.resolve(
        decision.kernel,
        decision.collision,
        decision.velocity,
        decision.authority_reason,
    );
    motion.set_authority(decision.authority);

    if regime_changed || execution_changed || previous_authority != motion.authority() {
        let reason = if decision.reason == LocomotionTransitionReason::AutomaticPolicy
            && decision.kernel == MotionKernel::InertialFlight
            && (assistance.mode() == TravelAssistance::Cruise
                || (previous_collision == CollisionPolicy::Disabled && developer_motion.is_none()))
        {
            LocomotionTransitionReason::NavigationAssistance
        } else {
            decision.reason
        };
        transitions.write(ControlledSubjectLocomotionChanged {
            entity,
            previous_regime,
            regime: locomotion.regime(),
            previous_kernel,
            kernel: execution.kernel(),
            reason,
            velocity_semantics: execution.velocity_semantics(),
            previous_authority,
            authority: motion.authority(),
            before,
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
            &MotionExecution,
            &LocomotionEnabled,
            Option<&CharacterMotor>,
            Option<&Collider>,
            Option<&DetailedBodyCollision>,
            &mut CharacterMovementIntent,
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
        execution,
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

    match execution.collision_policy() {
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

    let wants_character_motor = execution.kernel() == MotionKernel::Character;
    if wants_character_motor && motor.is_none() {
        commands.entity(entity).insert(CharacterMotor);
    } else if !wants_character_motor && motor.is_some() {
        commands.entity(entity).remove::<CharacterMotor>();
    }
}
