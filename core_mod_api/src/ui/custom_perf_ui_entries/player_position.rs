use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::player::components::Player;

#[derive(Component, Debug, Clone)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryPlayerPosition {
    pub label: String,
    pub sort_key: i32,
}

impl Default for PerfUiEntryPlayerPosition {
    fn default() -> Self {
        Self {
            label: "PlayerPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiEntryPlayerPosition {
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
        format!("({:.1}, {:.1}, {:.1})", value.x, value.y, value.z)
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        if value.x < -500.0 || value.x >= 500.0 || value.y < -500.0 || value.y >= 500.0  {
            Some(Color::linear_rgba(1.0, 0.0, 0.0, 1.0))
        } else {
            Some(Color::linear_rgba(0.0, 1.0, 0.0, 1.0))
        }
    }

    fn value_highlight(&self, value: &Self::Value) -> bool {
        value.x < -500.0 || value.x >= 500.0 || value.y < -500.0 || value.y >= 500.0
    }
}
