use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    CharacterGroundState, CharacterMovementConfig, CharacterMovementInput,
    controller::simulate_character_motors,
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
    CharacterGroundState
)]
pub struct CharacterMotor;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterMovementSet {
    Simulate,
}

pub struct CharacterMovementPlugin;

impl Plugin for CharacterMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharacterMotor>()
            .register_type::<CharacterMovementConfig>()
            .register_type::<CharacterMovementInput>()
            .register_type::<CharacterGroundState>()
            .configure_sets(FixedUpdate, CharacterMovementSet::Simulate)
            .add_systems(
                FixedUpdate,
                simulate_character_motors.in_set(CharacterMovementSet::Simulate),
            );
    }
}
