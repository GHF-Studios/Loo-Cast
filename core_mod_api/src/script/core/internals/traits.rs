use bevy::ecs::{system::{Commands, EntityCommands}, world::World};
use rhai::Shared;

use super::types::ScopedAccess;

pub trait AccessProvider<T> {
    fn start_access(&mut self) -> Shared<ScopedAccess<T>>;
    fn end_access(&mut self, access: Shared<ScopedAccess<T>>) -> Result<T, &'static str>;
}

impl AccessProvider<Commands> for World {
    fn start_access(&mut self) -> Shared<ScopedAccess<Commands>> {
        
    }
}
impl AccessProvider<EntityCommands<'_>> for Commands {
    fn start_access(&mut self) -> Shared<ScopedAccess<EntityCommands>> {
        
    }
}