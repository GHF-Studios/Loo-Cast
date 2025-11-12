use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput, PointerLocation, PointerPress};
use bevy::window::PrimaryWindow;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::{camera::components::MainCamera, picking::constants::MOUSE_POINTER_ID};

#[derive(Bundle, Default, Reflect)]
pub struct PerfUiCursorPosEntries {
    pub screen_pos: PerfUiEntryCursorScreenPos,
    pub unit_pos: PerfUiEntryCursorUnitPos,
    pub ingame_picking_screen_pos: PerfUiEntryGamePickingCursorScreenPos,
    pub indebug_picking_screen_pos: PerfUiEntryDebugPickingCursorScreenPos,
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

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryGamePickingCursorScreenPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryGamePickingCursorScreenPos {
    fn default() -> Self {
        Self {
            label: "(GPC)Pos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryGamePickingCursorScreenPos {
    type SystemParam = EventReader<'static, 'static, Pointer<Move>>;
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
        static PREVIOUS_POS: std::sync::Mutex<Option<Vec2>> = std::sync::Mutex::new(None);
        
        let mut previous_position = PREVIOUS_POS.lock().unwrap();

        let maybe_pos = query
            .read()
            .filter(|e| e.pointer_id == PointerId::Mouse)
            .last()
            .and_then(|event| event.hit.position.map(|p| p.truncate()));

        match (maybe_pos, *previous_position) {
            (None, None) => None,
            (Some(pos), None) => {
                *previous_position = Some(pos);
                Some(pos)
            },
            (None, Some(prev_pos)) => {
                *previous_position = None;
                Some(prev_pos)
            },
            (Some(pos), Some(_)) => {
                *previous_position = Some(pos);
                Some(pos)
            },
        }
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{:.1}", value)
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        Some(Color::linear_rgba(1.0, 0.0, 0.0, 1.0))
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryDebugPickingCursorScreenPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryDebugPickingCursorScreenPos {
    fn default() -> Self {
        Self {
            label: "(DPC)Pos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryDebugPickingCursorScreenPos {
    type SystemParam = EventReader<'static, 'static, Pointer<Move>>;
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
        static PREVIOUS_POS: std::sync::Mutex<Option<Vec2>> = std::sync::Mutex::new(None);

        let mut previous_position = PREVIOUS_POS.lock().unwrap();

        let maybe_pos = query
            .read()
            .filter(|e| e.pointer_id == MOUSE_POINTER_ID)
            .last()
            .and_then(|event| event.hit.position.map(|p| p.truncate()));

        match (maybe_pos, *previous_position) {
            (None, None) => None,
            (Some(pos), None) => {
                *previous_position = Some(pos);
                Some(pos)
            },
            (None, Some(prev_pos)) => {
                *previous_position = None;
                Some(prev_pos)
            },
            (Some(pos), Some(_)) => {
                *previous_position = Some(pos);
                Some(pos)
            },
        }
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{:.1}", value)
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        Some(Color::linear_rgba(1.0, 0.0, 0.0, 1.0))
    }

    fn value_highlight(&self, _value: &Self::Value) -> bool {
        false
    }
}
