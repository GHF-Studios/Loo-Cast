use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::camera::components::MainCamera;

#[derive(Bundle, Default, Reflect)]
pub struct PerfUiCursorPosEntries {
    pub screen_pos: PerfUiEntryCursorScreenPos,
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
        Query<'static, 'static, &'static Window>, 
        Query<'static, 'static, (&'static Camera, &'static GlobalTransform, &'static Projection), With<MainCamera>>,
    );
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
        let window = query.0.single().ok()?;
        let (camera, camera_transform, projection) = query.1.single().ok()?;

        let cursor_pos = window.cursor_position()?;
        // Step 1: Get the window size
        let window_size = Vec2::new(window.width(), window.height());

        // Step 2: Convert screen-space to NDC (-1.0 to +1.0)
        let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

        // Flip Y because screen-space Y is top-down, NDC is bottom-up
        let ndc = Vec3::new(ndc.x, -ndc.y, 1.0);

        let projection_matrix = match projection {
            Projection::Orthographic(orthographic) => orthographic.get_projection_matrix(),
            _ => return None, // Only orthographic projection is supported here
        };

        // Step 3: Transform NDC into world-space
        let ndc_to_world = camera_transform.compute_matrix() * inverse_projection_matrix;

        let world_pos = ndc_to_world.project_point3(ndc);

        Some(world_pos)
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
