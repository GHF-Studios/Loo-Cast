use crate::bevy::prelude::Commands as BevyCommands;
use crate::rhai_binding::value_semantics::scoped_access::ScopedAccessHandle;

#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: ScopedAccessHandle<BevyCommands<'static, 'static>>
}

#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: ScopedAccessHandle<crate::bevy::prelude::EntityCommands<'static>>
}
