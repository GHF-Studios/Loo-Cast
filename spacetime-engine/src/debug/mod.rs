//! Engine-wide developer observability.
//!
//! Debug producers live beside the domains they understand. This module owns
//! only shared view state and rendering conventions so simulation never needs
//! to know whether its state is currently being visualized.

mod color;
mod field;
mod menu;
mod metrics;
mod text;

pub use color::{DebugColorRamp, DebugColorStop, DebugScalarRange};
pub use field::{DebugScalarField2d, DebugVectorField2d, DebugVectorSpace};
pub use menu::DebugMenuState;
pub use text::{DebugCamera, billboard_text};

use std::any::TypeId;

use bevy::prelude::*;

/// Depth-tested developer gizmos representing world geometry/state.
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DebugGizmos;

/// Explanatory overlays that should remain visible through ordinary geometry.
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct DebugOverlayGizmos;

/// One independently controllable debug visualization / debug-equipment item.
pub trait DebugView: Send + Sync + 'static {
    const NAME: &'static str;
    const DESCRIPTION: &'static str = "";
    const ENABLED_BY_DEFAULT: bool = true;
}

#[derive(Debug, Clone, Copy)]
pub struct DebugViewInfo {
    pub type_id: TypeId,
    pub name: &'static str,
    pub description: &'static str,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy)]
struct DebugViewState {
    type_id: TypeId,
    name: &'static str,
    description: &'static str,
    enabled: bool,
}

/// Runtime state shared by every debug adapter.
///
/// Views are registered in deterministic plugin order. The registry is also the
/// backing model for the in-game debug-equipment menu, so new engine/mod views
/// become configurable without editing the menu itself.
#[derive(Resource, Debug)]
pub struct DebugViews {
    master_enabled: bool,
    solo: Option<TypeId>,
    views: Vec<DebugViewState>,
}

impl Default for DebugViews {
    fn default() -> Self {
        Self {
            master_enabled: true,
            solo: None,
            views: Vec::new(),
        }
    }
}

impl DebugViews {
    pub fn master_enabled(&self) -> bool {
        self.master_enabled
    }

    pub fn set_master_enabled(&mut self, enabled: bool) {
        self.master_enabled = enabled;
    }

    pub fn toggle_master(&mut self) -> bool {
        self.master_enabled = !self.master_enabled;
        self.master_enabled
    }

    pub fn enabled<V: DebugView>(&self) -> bool {
        if !self.master_enabled {
            return false;
        }

        let type_id = TypeId::of::<V>();
        if self.solo.is_some_and(|solo| solo != type_id) {
            return false;
        }

        self.views
            .iter()
            .find(|view| view.type_id == type_id)
            .is_some_and(|view| view.enabled)
    }

    pub fn set<V: DebugView>(&mut self, enabled: bool) {
        self.register::<V>();
        let type_id = TypeId::of::<V>();
        if let Some(view) = self.views.iter_mut().find(|view| view.type_id == type_id) {
            view.enabled = enabled;
        }
    }

    pub fn toggle<V: DebugView>(&mut self) -> bool {
        self.register::<V>();
        let type_id = TypeId::of::<V>();
        let view = self
            .views
            .iter_mut()
            .find(|view| view.type_id == type_id)
            .expect("registered debug view disappeared");
        view.enabled = !view.enabled;
        view.enabled
    }

    pub fn toggle_type_id(&mut self, type_id: TypeId) -> Option<bool> {
        let view = self.views.iter_mut().find(|view| view.type_id == type_id)?;
        view.enabled = !view.enabled;
        Some(view.enabled)
    }

    pub fn view(&self, type_id: TypeId) -> Option<DebugViewInfo> {
        self.views
            .iter()
            .find(|view| view.type_id == type_id)
            .map(|view| DebugViewInfo {
                type_id: view.type_id,
                name: view.name,
                description: view.description,
                enabled: view.enabled,
            })
    }

    pub fn views(&self) -> impl ExactSizeIterator<Item = DebugViewInfo> + '_ {
        self.views.iter().map(|view| DebugViewInfo {
            type_id: view.type_id,
            name: view.name,
            description: view.description,
            enabled: view.enabled,
        })
    }

    /// Optional temporary focus retained as a useful programmatic/tooling
    /// operation even though the menu now provides the primary UI.
    pub fn cycle_focus(&mut self) -> Option<&'static str> {
        if self.views.is_empty() {
            self.solo = None;
            return None;
        }

        self.solo = match self.solo {
            None => Some(self.views[0].type_id),
            Some(current) => {
                let next = self
                    .views
                    .iter()
                    .position(|view| view.type_id == current)
                    .and_then(|index| self.views.get(index + 1));
                next.map(|view| view.type_id)
            }
        };

        self.solo.and_then(|solo| {
            self.views
                .iter()
                .find(|view| view.type_id == solo)
                .map(|view| view.name)
        })
    }

    fn register<V: DebugView>(&mut self) {
        let type_id = TypeId::of::<V>();
        if self.views.iter().any(|view| view.type_id == type_id) {
            return;
        }

        self.views.push(DebugViewState {
            type_id,
            name: V::NAME,
            description: V::DESCRIPTION,
            enabled: V::ENABLED_BY_DEFAULT,
        });
    }
}

pub trait AppDebugExt {
    fn register_debug_view<V: DebugView>(&mut self) -> &mut Self;
}

impl AppDebugExt for App {
    fn register_debug_view<V: DebugView>(&mut self) -> &mut Self {
        self.init_resource::<DebugViews>();
        self.world_mut().resource_mut::<DebugViews>().register::<V>();
        self
    }
}

/// Installs the shared debug substrate and the domain adapters currently owned
/// by Spacetime Engine / the pressure-test game.
pub struct SpacetimeDebugPlugin;

impl Plugin for SpacetimeDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugViews>()
            .init_gizmo_group::<DebugGizmos>()
            .init_gizmo_group::<DebugOverlayGizmos>()
            .add_systems(Startup, configure_debug_gizmos);

        // Generic render/instrumentation backends first so domain adapters can
        // simply publish fields and views into them.
        field::configure(app);
        metrics::configure(app);

        crate::ecs::debug::configure(app);
        crate::physics::debug::configure(app);
        crate::physics::character::debug::configure(app);
        crate::game::portal::debug::configure(app);
        crate::game::thermal::debug::configure(app);

        // Build the menu last so every registered view appears as an item.
        menu::configure(app);
    }
}

fn configure_debug_gizmos(mut store: ResMut<GizmoConfigStore>) {
    let (world, _) = store.config_mut::<DebugGizmos>();
    world.line.width = 2.0;

    let (overlay, _) = store.config_mut::<DebugOverlayGizmos>();
    overlay.line.width = 2.0;
    overlay.depth_bias = -1.0;
}
