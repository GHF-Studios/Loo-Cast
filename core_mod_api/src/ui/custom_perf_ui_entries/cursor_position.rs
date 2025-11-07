use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::window::PrimaryWindow;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::camera::components::MainCamera;

#[derive(Bundle, Default, Reflect)]
pub struct PerfUiCursorPosEntries {
    pub screen_pos: PerfUiEntryCursorScreenPos,
    pub unit_pos: PerfUiEntryCursorUnitPos,
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryCursorScreenPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryCursorScreenPos {
    fn default() -> Self {
        Self {
            label: "(C)ScreenPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryCursorScreenPos {
    type SystemParam = Query<'static, 'static, &'static Window>;
    type Value = Vec2;

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
        let window = query.single().ok()?;
        window.cursor_position()
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

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryCursorUnitPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryCursorUnitPos {
    fn default() -> Self {
        Self {
            label: "(C)UnitPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryCursorUnitPos {
    type SystemParam = (
        Query<'static, 'static, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
        Query<'static, 'static, &'static Window, With<PrimaryWindow>>,
    );
    type Value = Vec2;

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
        let (camera, camera_transform) = query.0.single().ok()?;
        let window = query.1.single().ok()?;

        window.cursor_position()
            .map(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .and_then(|ray| ray.map(|r| r.origin.truncate()))
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
