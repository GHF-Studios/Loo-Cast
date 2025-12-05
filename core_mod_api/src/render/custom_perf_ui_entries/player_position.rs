use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::chunk_actor::components::ChunkActor;
use crate::player::components::Player;
use crate::usf::pos::grid::types::GridVec;

#[derive(Bundle, Default, Reflect)]
pub struct PerfUiPlayerPosEntries {
    pub grid_offset: PerfUiEntryPlayerGridPos,
    pub unit_offset: PerfUiEntryPlayerUnitPos,
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryPlayerGridPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryPlayerGridPos {
    fn default() -> Self {
        Self {
            label: "(P)GridPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryPlayerGridPos {
    type SystemParam = Query<'static, 'static, &'static ChunkActor, With<Player>>;
    type Value = GridVec;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        query.iter().next().map(|chunk_actor| chunk_actor.coord.clone())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{:?}", value)
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        Some(Color::linear_rgba(0.0, 0.0, 1.0, 1.0))
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryPlayerUnitPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryPlayerUnitPos {
    fn default() -> Self {
        Self {
            label: "(P)UnitPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryPlayerUnitPos {
    type SystemParam = Query<'static, 'static, &'static Transform, With<Player>>;
    type Value = Vec3;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        query.iter().next().map(|transform| transform.translation)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{:.1}", value)
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        Some(Color::linear_rgba(0.0, 0.0, 1.0, 1.0))
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}
