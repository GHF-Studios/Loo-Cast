pub mod functions;
pub mod resources;
pub mod statics;
pub mod systems;
pub mod types;

pub mod tracing;
pub mod ui;

use bevy::prelude::*;

use resources::LogRegistry;
use types::*;

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((tracing::TracingPlugin, ui::UiPlugin))
            .insert_resource(LogRegistry::default())
            .register_type::<LogLevel>()
            .register_type::<LogEntry>()
            .register_type::<LogId>()
            .register_type::<LogPath>()
            .register_type::<ExplicitSelectionState>()
            .register_type::<EffectiveSelectionState>()
            .register_type::<NodeMetadata>()
            .register_type::<SpanPath>()
            .register_type::<ModulePath>()
            .register_type::<PhysicalStoragePath>()
            .register_type::<PhysicalSelectionPath>()
            .register_type::<SpanSegment>()
            .register_type::<CrateModuleSegment>()
            .register_type::<ModuleSegment>()
            .register_type::<SubModuleSegment>()
            .register_type::<CrateFolderSegment>()
            .register_type::<FolderSegment>()
            .register_type::<FileSegment>()
            .register_type::<LineSegment>()
            .register_type::<SpanRegistry>()
            .register_type::<ModuleRegistry>()
            .register_type::<PhysicalRegistry>()
            .register_type::<SpanNode>()
            .register_type::<CrateModuleNode>()
            .register_type::<ModuleNode>()
            .register_type::<SubModuleNode>()
            .register_type::<CrateFolderNode>()
            .register_type::<FolderNode>()
            .register_type::<FileNode>()
            .register_type::<LineNode>()
            .register_type::<SpanPathSelections>()
            .register_type::<ModulePathSelections>()
            .register_type::<PhysicalPathSelections>()
            .register_type::<SpanNodeSelection>()
            .register_type::<CrateModuleNodeSelection>()
            .register_type::<ModuleNodeSelection>()
            .register_type::<SubModuleNodeSelection>()
            .register_type::<CrateFolderNodeSelection>()
            .register_type::<FolderNodeSelection>()
            .register_type::<FileNodeSelection>()
            .register_type::<LineNodeSelection>();
    }
}
