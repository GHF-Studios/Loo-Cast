use bevy::prelude::*;
use bevy::picking::pointer::PointerId;
use uuid::Uuid;

pub const MOUSE_POINTER_ID: PointerId = PointerId::Custom(Uuid::from_u128(0u128));
pub const NO_HIT_SENTINEL: Entity = Entity::from_raw(u32::MAX);
