use rhai::{FnPtr, NativeCallContext};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::script::ecs::entity::bindings::types::EntityId;

#[derive(Clone)]
#[repr(transparent)]
pub struct Commands {
    commands: Option<Arc<Mutex<bevy::prelude::Commands<'static, 'static>>>>
}
impl Commands {
    pub(in crate::script) fn start_access<'w, 's>(source: bevy::prelude::Commands<'w, 's>) -> Self {
        let static_source: bevy::prelude::Commands<'static, 'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            commands: Some(Arc::new(Mutex::new(static_source))),
        }
    }

    pub(in crate::script) fn end_access<'w, 's>(mut self) -> bevy::prelude::Commands<'w, 's> {
        let commands = self.commands.take().expect("Already cleaned up!");
        let commands = Arc::into_inner(commands).expect("Too many refs!");
        let commands = commands.into_inner().unwrap();
        
        unsafe {
            std::mem::transmute(commands)
        }
    }

    pub(in crate::script) fn raw_access<'w, 's>(&'_ self) -> MutexGuard<'_, bevy::prelude::Commands<'w, 's>> {
        let commands = self.commands.as_ref().unwrap().lock().unwrap();

        unsafe {
            std::mem::transmute(commands)
        }
    }

    pub fn entity_commands(&mut self, ctx: NativeCallContext, entity_id: EntityId, f: FnPtr) {
        let entity_commands = EntityCommands::start_access(self.raw_access().entity(entity_id.to_raw()));
        let entity_commands: EntityCommands = f.call_within_context(&ctx, (entity_commands,)).unwrap();
        let _ = entity_commands.end_access();
    }
}
impl Drop for Commands {
    fn drop(&mut self) {
        if self.commands.is_some() {
            panic!("This type should not be copied/cloned!")
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityCommands {
    entity_commands: Option<Arc<Mutex<bevy::prelude::EntityCommands<'static>>>>
}
impl EntityCommands {
    pub(in super::super) fn start_access<'a>(source: bevy::prelude::EntityCommands<'a>) -> Self {
        let static_source: bevy::prelude::EntityCommands<'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            entity_commands: Some(Arc::new(Mutex::new(static_source))),
        }
    }

    pub(in super::super) fn end_access<'a>(mut self) -> bevy::prelude::EntityCommands<'a> {
        let entity_commands = self.entity_commands.take().expect("Already cleaned up!");
        let entity_commands = Arc::into_inner(entity_commands).expect("Too many refs!");
        let entity_commands = entity_commands.into_inner().unwrap();
        
        unsafe {
            std::mem::transmute(entity_commands)
        }
    }

    pub(in super::super) fn raw_access<'a>(&'_ self) -> MutexGuard<'_, bevy::prelude::EntityCommands<'a>> {
        let entity_commands = self.entity_commands.as_ref().unwrap().lock().unwrap();

        unsafe {
            std::mem::transmute(entity_commands)
        }
    }

    pub fn commands(&mut self, ctx: NativeCallContext, f: FnPtr) {
        let commands = Commands::start_access(self.raw_access().commands());
        let commands: Commands = f.call_within_context(&ctx, (commands,)).unwrap();
        let _ = commands.end_access();
    }
}
