use bevy::prelude::Commands as BevyCommands;
use core_mod_core::reflection::access::ScopedAccessHandle;
use rhai::{Dynamic, FnPtr, NativeCallContext};

use crate::script::core::internals::traits::ScopedAccessProvider;

#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: ScopedAccessHandle<BevyCommands<'static, 'static>>
}

#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: ScopedAccessHandle<bevy::prelude::EntityCommands<'static>>
}