//! Thermal test tool.
//!
//! This tool injects signed thermal energy. It deliberately does not know how
//! ignition works and never inserts `Combustion` itself.

use avian3d::prelude::SpatialQuery;
use bevy::prelude::*;

use crate::{
    game::{
        GameSet,
        item::{ItemAction, ItemActionHint, ItemCatalog, ItemDefinition, ItemId, UseItem},
    },
    physics::topology::UsfRuntimeOwnershipQuery,
    thermal::ThermalPointImpulse,
};

pub const HEAT_RAY: ItemId = ItemId::new("heat_ray");

const RANGE: f32 = 100.0;
const HEAT_ENERGY_JOULES: f32 = 220_000.0;
const COOL_ENERGY_JOULES: f32 = -220_000.0;

pub struct HeatRayItemPlugin;

impl Plugin for HeatRayItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Update, use_heat_ray.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<ItemCatalog>) {
    catalog.register(ItemDefinition {
        id: HEAT_RAY,
        name: "Heat Ray",
        description: "LMB injects heat at the hit point; RMB removes it. Internal gradients are systemic.",
        action_hints: vec![
            ItemActionHint::new(ItemAction::PRIMARY, "Heat target"),
            ItemActionHint::new(ItemAction::SECONDARY, "Cool target"),
        ],
    });
}

fn use_heat_ray(
    mut uses: MessageReader<UseItem>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    spatial_query: SpatialQuery,
    mut impulses: MessageWriter<ThermalPointImpulse>,
) {
    for request in uses.read() {
        if request.item != HEAT_RAY {
            continue;
        }

        let energy_joules = if request.action == ItemAction::PRIMARY {
            HEAT_ENERGY_JOULES
        } else if request.action == ItemAction::SECONDARY {
            COOL_ENERGY_JOULES
        } else {
            continue;
        };

        let Ok(direction) = Dir3::new(request.aim.direction) else {
            continue;
        };

        let filter = runtime_ownership.filter_excluding_subject(request.actor);

        let Some(hit) =
            spatial_query.cast_ray(request.aim.origin, direction, RANGE, false, &filter)
        else {
            continue;
        };

        let hit_point = request.aim.origin + request.aim.direction * hit.distance;
        impulses.write(ThermalPointImpulse {
            target: hit.entity,
            world_position: hit_point,
            energy_joules,
        });
    }
}
