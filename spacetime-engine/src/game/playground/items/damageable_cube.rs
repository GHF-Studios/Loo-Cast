use bevy::prelude::*;

use crate::{
    ecs::{
        UsfEntity,
        UsfManifestationOf,
    },
    game::{
        GameAssets,
        GameSet,
        combat::{
            Health,
            Hitbox,
        },
    },
};

use super::super::{
    catalog::{
        PlaygroundCatalog,
        PlaygroundItem,
        PlaygroundItemId,
        UsePlaygroundItem,
    },
    object::{
        PlaygroundPickable,
        PlaygroundRoot,
        ShowHealthInPlaygroundHud,
    },
};

pub const DAMAGEABLE_CUBE:
    PlaygroundItemId =
    PlaygroundItemId::new(
        "damageable_cube",
    );

pub const SPLIT_DAMAGEABLE_CUBE:
    PlaygroundItemId =
    PlaygroundItemId::new(
        "split_damageable_cube",
    );

const CUBE_SIZE: f32 = 1.0;
const MAXIMUM_HEALTH: f32 = 100.0;
const PLACEMENT_DISTANCE: f32 = 100.0;

#[derive(Resource, Default)]
struct CubeCounter(u64);

pub struct DamageableCubeItemPlugin;

impl Plugin for DamageableCubeItemPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_resource::<CubeCounter>()
            .add_systems(
                PreStartup,
                register_items,
            )
            .add_systems(
                Update,
                use_cube_items
                    .in_set(
                        GameSet::Action,
                    ),
            );
    }
}

fn register_items(
    mut catalog:
        ResMut<PlaygroundCatalog>,
) {
    catalog.register(
        PlaygroundItem {
            id: DAMAGEABLE_CUBE,
            name: "Damageable Cube",
            description:
                "One semantic Health owner with one visible manifestation.",
        },
    );

    catalog.register(
        PlaygroundItem {
            id:
                SPLIT_DAMAGEABLE_CUBE,
            name:
                "Split Damageable Cube",
            description:
                "One semantic Health owner with two symmetric manifestations.",
        },
    );
}

fn use_cube_items(
    mut commands: Commands,
    mut uses:
        MessageReader<
            UsePlaygroundItem,
        >,
    assets:
        Res<GameAssets>,
    mut counter:
        ResMut<CubeCounter>,
) {
    for request in uses.read() {
        let manifestation_offsets:
            &[Vec3] =
            match request.item {
                DAMAGEABLE_CUBE => {
                    &[Vec3::ZERO]
                }

                SPLIT_DAMAGEABLE_CUBE => {
                    &[
                        Vec3::new(
                            -1.25,
                            0.0,
                            0.0,
                        ),
                        Vec3::new(
                            1.25,
                            0.0,
                            0.0,
                        ),
                    ]
                }

                _ => continue,
            };

        let Some(ground) =
            request
                .aim
                .horizontal_plane(
                    0.0,
                    PLACEMENT_DISTANCE,
                )
        else {
            continue;
        };

        counter.0 += 1;

        let logical_name =
            if request.item
                == DAMAGEABLE_CUBE
            {
                format!(
                    "Cube {}",
                    counter.0,
                )
            } else {
                format!(
                    "Split Cube {}",
                    counter.0,
                )
            };

        let root =
            commands
                .spawn((
                    Name::new(
                        logical_name,
                    ),
                    PlaygroundRoot,
                    ShowHealthInPlaygroundHud,
                    UsfEntity,
                    Health::new(
                        MAXIMUM_HEALTH,
                    ),
                ))
                .id();

        for (
            index,
            offset,
        ) in manifestation_offsets
            .iter()
            .copied()
            .enumerate()
        {
            let position =
                ground
                    + Vec3::Y
                        * (
                            CUBE_SIZE
                                / 2.0
                        )
                    + offset;

            commands.spawn((
                Name::new(
                    format!(
                        "Cube Manifestation {}",
                        index,
                    ),
                ),
                UsfManifestationOf(
                    root,
                ),
                PlaygroundPickable::cube(
                    root,
                    CUBE_SIZE,
                ),
                Hitbox::cube(
                    CUBE_SIZE,
                ),
                Mesh3d(
                    assets
                        .damageable_cube_mesh
                        .clone(),
                ),
                MeshMaterial3d(
                    assets
                        .damageable_cube_material
                        .clone(),
                ),
                Transform::from_translation(
                    position,
                ),
            ));
        }
    }
}
