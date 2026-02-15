use crate::bevy::prelude::Commands as BevyCommands;
use crate::script::access::ScopedAccessHandle;
use rhai::{Dynamic, FnPtr, NativeCallContext};

use crate::reflection::internals::traits::ScopedAccessProvider;

#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: ScopedAccessHandle<BevyCommands<'static, 'static>>
}

#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: ScopedAccessHandle<crate::bevy::prelude::EntityCommands<'static>>
}