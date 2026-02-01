use bevy::picking::pointer::PointerId;
use bevy::prelude::*;
use uuid::Uuid;

pub const DIEGETIC_MOUSE_POINTER_ID: PointerId = PointerId::Custom(Uuid::from_u128(0u128));
pub const META_MOUSE_POINTER_ID: PointerId = PointerId::Custom(Uuid::from_u128(1u128));
pub const NO_HIT_SENTINEL: Entity = Entity::from_raw_u32(u32::MAX - 1000u32).unwrap();
