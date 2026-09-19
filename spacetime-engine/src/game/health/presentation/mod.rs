use std::collections::{HashMap, HashSet};

use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    transform::TransformSystems,
};

use crate::{
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        player::{Player, PlayerCamera},
    },
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{Health, Hitbox};

const PLAYER_BAR_WIDTH_PX: f32 = 320.0;
const PLAYER_BAR_HEIGHT_PX: f32 = 16.0;
const PLAYER_BAR_BOTTOM_PX: f32 = 78.0;
const PLAYER_BAR_BORDER_PX: f32 = 2.0;

const WORLD_BAR_WIDTH: f32 = 1.1;
const WORLD_BAR_HEIGHT: f32 = 0.11;
const WORLD_BAR_DEPTH: f32 = 0.025;
const WORLD_BAR_BORDER: f32 = 0.018;
const WORLD_BAR_GAP: f32 = 0.28;
const WORLD_BAR_DEFAULT_HALF_HEIGHT: f32 = 0.5;
const WORLD_BAR_FILL_Z_OFFSET: f32 = 0.02;

const FRAME_COLOR: Color = Color::srgb(0.035, 0.035, 0.04);
const FRAME_BORDER_COLOR: Color = Color::srgb(0.65, 0.65, 0.68);
const FILL_COLOR: Color = Color::srgb(0.82, 0.025, 0.035);

mod player;
mod world;

use player::{spawn_player_health_bar, sync_player_health_bar};
use world::{setup_world_health_bar_assets, sync_world_health_bars, WorldHealthBarCache};

pub(super) fn configure(app: &mut App) {
    app.init_resource::<WorldHealthBarCache>()
        .add_systems(
            Startup,
            (spawn_player_health_bar, setup_world_health_bar_assets),
        )
        .add_systems(Update, sync_player_health_bar.in_set(GameSet::Presentation))
        .add_systems(
            PostUpdate,
            sync_world_health_bars.after(TransformSystems::Propagate),
        );
}

fn health_fraction(health: &Health) -> f32 {
    (health.current() / health.maximum()).clamp(0.0, 1.0)
}
