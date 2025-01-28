use bevy::prelude::*;

pub trait ActionTarget: Send + Sync {
    fn get_entity(&self) -> Entity;
}
