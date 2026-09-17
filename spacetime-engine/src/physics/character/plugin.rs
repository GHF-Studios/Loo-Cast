use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    CharacterControlFrame, CharacterGroundState, CharacterLocomotionFrame, CharacterMovementConfig,
    CharacterMovementInput,
    controller::{
        CharacterPush, apply_character_pushes, receive_dynamic_contact_pushes,
        simulate_character_motors,
    },
    frame::settle_character_control_frames,
};

/// Marker for an entity whose transform is integrated by the character motor.
///
/// A [`Collider`] must be supplied explicitly because hull shape is a gameplay
/// choice rather than an engine default.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    RigidBody::Kinematic,
    CustomPositionIntegration,
    CustomVelocityIntegration,
    SpeculativeMargin(0.0),
    Transform,
    LinearVelocity,
    CharacterMovementConfig,
    CharacterMovementInput,
    CharacterGroundState,
    CharacterLocomotionFrame,
    CharacterControlFrame
)]
pub struct CharacterMotor;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterMovementSet {
    Simulate,
    PushDynamics,
    ReceiveDynamics,
}

pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CharacterPush>()
            .register_type::<CharacterMotor>()
            .register_type::<CharacterMovementConfig>()
            .register_type::<CharacterMovementInput>()
            .register_type::<CharacterGroundState>()
            .register_type::<CharacterLocomotionFrame>()
            .register_type::<CharacterControlFrame>()
            .add_systems(PreUpdate, settle_character_control_frames)
            .configure_sets(FixedUpdate, CharacterMovementSet::Simulate)
            .add_systems(
                FixedUpdate,
                simulate_character_motors.in_set(CharacterMovementSet::Simulate),
            )
            .add_systems(
                FixedPostUpdate,
                apply_character_pushes
                    .in_set(CharacterMovementSet::PushDynamics)
                    .in_set(PhysicsSystems::First),
            )
            .add_systems(
                FixedPostUpdate,
                receive_dynamic_contact_pushes
                    .in_set(CharacterMovementSet::ReceiveDynamics)
                    .after(PhysicsSystems::Last),
            );
    }
}
