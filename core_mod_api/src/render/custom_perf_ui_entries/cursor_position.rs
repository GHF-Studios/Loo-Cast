use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use iyes_perf_ui::{entry::PerfUiEntry, ui::root::PerfUiRoot, utils::next_sort_key};

use crate::render::components::MainCamera;
use crate::render::resources::{GameViewRenderTarget, PrimaryWindowUiState};

#[derive(Bundle, Default, Reflect)]
pub struct PerfUiCursorPosEntries {
    pub viewport_rect: PerfUiEntryViewportRect,
    pub window_pos: PerfUiEntryCursorWindowPos,
    pub pointer_pos: PerfUiEntryCursorPointerPos,
    pub unit_pos: PerfUiEntryCursorUnitPos,
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(PerfUiRoot)]
pub struct PerfUiEntryViewportRect {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryViewportRect {
    fn default() -> Self {
        Self {
            label: "ViewportRect".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryViewportRect {
    type SystemParam = Res<'static, PrimaryWindowUiState>;
    type Value = egui::Rect;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, sys_param: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let primary_window_ui_state = sys_param;

        primary_window_ui_state.viewport_rect_precision_proxy
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("({:.1}, {:.1})->({:.1}, {:.1})", value.min.x, value.min.y, value.max.x, value.max.y)
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
pub struct PerfUiEntryCursorWindowPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryCursorWindowPos {
    fn default() -> Self {
        Self {
            label: "(C)WindowPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryCursorWindowPos {
    type SystemParam = (
        Local<'static, Option<Vec2>>,
        Query<'static, 'static, &'static Window, With<PrimaryWindow>>,
        Res<'static, GameViewRenderTarget>,
        Res<'static, PrimaryWindowUiState>,
    );
    type Value = Vec2;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, sys_params: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let (ref mut previous_position, ref window_query, ref game_view_render_target, ref primary_window_ui_state) = sys_params;

        let window = window_query.single().ok()?;

        let window_size = window.physical_size();
        let window_size_vec2 = Vec2::new(window_size.x as f32, window_size.y as f32);
        let viewport_size = game_view_render_target.size;
        let viewport_size_vec2 = Vec2::new(viewport_size.x as f32, viewport_size.y as f32);

        **previous_position = if let Some(current_position) = window.cursor_position() {
            let viewport_rect = primary_window_ui_state.viewport_rect_precision_proxy?;

            if viewport_rect.contains(egui::Pos2 {
                x: current_position.x,
                y: current_position.y,
            }) {
                let x = current_position.x.remap(viewport_rect.min.x, viewport_rect.max.x, 0.0, window_size_vec2.x);
                let y = current_position.y.remap(viewport_rect.min.y, viewport_rect.max.y, 0.0, window_size_vec2.y);
                Some(Vec2::new(x, y))
            } else {
                None
            }
        } else {
            None
        };

        **previous_position
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
pub struct PerfUiEntryCursorPointerPos {
    pub label: String,
    pub sort_key: i32,
}
impl Default for PerfUiEntryCursorPointerPos {
    fn default() -> Self {
        Self {
            label: "(C)PointerPos".to_string(),
            sort_key: next_sort_key(),
        }
    }
}
impl PerfUiEntry for PerfUiEntryCursorPointerPos {
    type SystemParam = (
        Local<'static, Option<Vec2>>,
        EventReader<'static, 'static, Pointer<Move>>,
        Query<'static, 'static, &'static Window, With<PrimaryWindow>>,
        Res<'static, GameViewRenderTarget>,
        Res<'static, PrimaryWindowUiState>,
    );
    type Value = Vec2;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, sys_params: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let (ref mut previous_position, ref mut pointer_move_events, ref window_query, ref game_view_render_target, ref primary_window_ui_state) = sys_params;

        let window = window_query.single().ok()?;

        let window_size = window.physical_size();
        let window_size_vec2 = Vec2::new(window_size.x as f32, window_size.y as f32);
        let viewport_size = game_view_render_target.size;
        let viewport_size_vec2 = Vec2::new(viewport_size.x as f32, viewport_size.y as f32);

        **previous_position = if let Some(event) = pointer_move_events.read().last() {
            let current_position = event.pointer_location.position;
            let viewport_rect = primary_window_ui_state.viewport_rect_precision_proxy?;

            if viewport_rect.contains(egui::Pos2 {
                x: current_position.x,
                y: current_position.y,
            }) {
                let x = current_position.x.remap(viewport_rect.min.x, viewport_rect.max.x, 0.0, window_size_vec2.x);
                let y = current_position.y.remap(viewport_rect.min.y, viewport_rect.max.y, 0.0, window_size_vec2.y);
                Some(Vec2::new(x, y))
            } else {
                None
            }
        } else {
            None
        };

        **previous_position
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
        Local<'static, Option<Vec2>>,
        Query<'static, 'static, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
        Query<'static, 'static, &'static Window, With<PrimaryWindow>>,
        Res<'static, GameViewRenderTarget>,
        Res<'static, PrimaryWindowUiState>,
    );
    type Value = Vec2;

    fn label(&self) -> &str {
        &self.label
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(&self, sys_params: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>) -> Option<Self::Value> {
        let (ref mut previous_position, ref camera_query, ref window_query, ref game_view_render_target, ref primary_window_ui_state) = sys_params;

        let (camera, camera_transform) = camera_query.single().ok()?;
        let window = window_query.single().ok()?;

        let window_size = window.physical_size();
        let window_size_vec2 = Vec2::new(window_size.x as f32, window_size.y as f32);
        let viewport_size = game_view_render_target.size;
        let viewport_size_vec2 = Vec2::new(viewport_size.x as f32, viewport_size.y as f32);

        **previous_position = if let Some(current_position) = window.cursor_position() {
            let viewport_rect = primary_window_ui_state.viewport_rect_precision_proxy?;

            if viewport_rect.contains(egui::Pos2 {
                x: current_position.x,
                y: current_position.y,
            }) {
                let x = current_position.x.remap(viewport_rect.min.x, viewport_rect.max.x, 0.0, window_size_vec2.x);
                let y = current_position.y.remap(viewport_rect.min.y, viewport_rect.max.y, 0.0, window_size_vec2.y);
                camera
                    .viewport_to_world(camera_transform, Vec2::new(x, y))
                    .ok()
                    .map(|ray| ray.origin.truncate())
            } else {
                None
            }
        } else {
            None
        };

        **previous_position
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
