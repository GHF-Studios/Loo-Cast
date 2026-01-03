use bevy::prelude::*;
use rhai::{FnPtr, NativeCallContext};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
#[repr(transparent)]
pub struct World {
    world: Option<Arc<Mutex<bevy::prelude::World>>>,
}
impl World {
    pub(in super::super) fn start_access(source: bevy::prelude::World) -> Self {
        Self {
            world: Some(Arc::new(Mutex::new(source))),
        }
    }

    pub(in super::super) fn end_access(mut self) -> bevy::prelude::World {
        let world = self.world.take().expect("Already cleaned up!");
        let world = Arc::into_inner(world).expect("Too many refs!");
        let world = world.into_inner().unwrap();

        world
    }

    pub(in super::super) fn raw_access(&'_ self) -> MutexGuard<'_, bevy::prelude::World> {
        self.world.as_ref().unwrap().lock().unwrap()
    }

    pub fn commands(&self, ctx: NativeCallContext, f: FnPtr) {
        let commands = Commands::start_access(self.raw_access().commands());
        let commands: Commands = f.call_within_context(&ctx, (commands,)).unwrap();
        let _ = commands.end_access();
    }

    pub fn flush(&self) {
        self.raw_access().flush();
    }

    // My personal note book; not used anymore, idk lol. Like writing on the back of a printout.
    #[deprecated]
    pub fn spawn_named_entity(&self, name: String) {
        // Irrelevant Notes
        self.raw_access().add_observer(system);
        self.raw_access().add_schedule(schedule);
        self.raw_access().add_asset(asset);
        self.raw_access().clear_all();
        self.raw_access().despawn(entity);
        self.raw_access().query();
        self.raw_access().query_filtered();
        self.raw_access().remove_resource();
        self.raw_access().removed();
        self.raw_access().run_schedule(label);
        self.raw_access().run_system(id);
        self.raw_access().spawn(bundle);

        self.raw_access().archetypes();
        self.raw_access().bundles();
        self.raw_access().components();
        self.raw_access().entity(entities);
        self.raw_access().entity_mut(entities);
        self.raw_access().entities();
        self.raw_access().get(entity);
        self.raw_access().get_mut(entity);
        self.raw_access().get_entity(entities);
        self.raw_access().get_entity_mut(entities);
        self.raw_access().get_resource();
        self.raw_access().get_resource_mut();
        self.raw_access().init_resource();
        self.raw_access().insert_resource(value);
        self.raw_access().storages();
    }
}
impl Drop for World {
    fn drop(&mut self) {
        if self.world.is_some() {
            panic!("World shall not be copied/cloned!")
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Commands {
    commands: Option<Arc<Mutex<bevy::prelude::Commands<'static, 'static>>>>
}
impl Commands {
    pub(in super::super) fn start_access<'w, 's>(source: bevy::prelude::Commands<'w, 's>) -> Self {
        let static_source: bevy::prelude::Commands<'static, 'static> = unsafe {
            std::mem::transmute(source)
        };

        Self {
            commands: Some(Arc::new(Mutex::new(static_source))),
        }
    }

    pub(in super::super) fn end_access<'w, 's>(mut self) -> bevy::prelude::Commands<'w, 's> {
        let commands = self.commands.take().expect("Already cleaned up!");
        let commands = Arc::into_inner(commands).expect("Too many refs!");
        let commands = commands.into_inner().unwrap();
        
        unsafe {
            std::mem::transmute(commands)
        }
    }

    pub(in super::super) fn raw_access<'w, 's>(&'_ self) -> MutexGuard<'_, bevy::prelude::Commands<'w, 's>> {
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
            panic!("Commands shall not be copied/cloned!")
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityId {
    entity_id: Entity
}
impl EntityId {
    pub fn from_raw(entity_id: Entity) -> Self {
        Self { entity_id }
    }

    pub fn to_raw(self) -> Entity {
        self.entity_id
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityRef {
    entity_mut: bevy::prelude::EntityRef<'static>
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityMut {
    entity_mut: Option<Arc<Mutex<bevy::prelude::EntityMut<'static>>>>
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityWorldMut {
    entity_world_mut: Option<Arc<Mutex<bevy::prelude::EntityWorldMut<'static>>>>
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

#[derive(Clone)]
#[repr(transparent)]
pub struct Component {
    bundle: Option<Arc<Mutex<something>>> // Excuse me, what the fuck
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Bundle {
    bundle: Option<Arc<Mutex<something>>> // Excuse me, what the fuck
}