use crate::bevy::prelude::Resource;

#[derive(Resource, Clone, Debug, Default)]
pub struct ScriptProbeResource {
    pub payload: String,
}
