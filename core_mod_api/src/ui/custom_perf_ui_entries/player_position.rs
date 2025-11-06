use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::chunk_actor::components::ChunkActor;
use crate::player::components::Player;
use crate::usf::pos::grid::types::GridVec;

#[derive(Bundle, Default)]
pub struct PerfUiEntryPlayerPosEntries {
    pub grid_offset: PerfUiEntryPlayerGridOffset,
    pub unit_offset: PerfUiEntryPlayerUnitOffset,
}

#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryPlayerGridOffset {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryPlayerGridOffset {
    fn default() -> Self {
        Self {
            label: "GridPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryPlayerGridOffset {
    type SystemParam = Query<'static, 'static, &'static ChunkActor, With<Player>>;
    type Value = GridVec;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
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

#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryPlayerUnitOffset {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryPlayerUnitOffset {
    fn default() -> Self {
        Self {
            label: "UnitPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryPlayerUnitOffset {
    type SystemParam = Query<'static, 'static, &'static Transform, With<Player>>;
    type Value = Vec3;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        query: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
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
