//! Space-Engineers-style voxel hand backed by the multi-chunk voxel world.

use bevy::{camera::visibility::NoFrustumCulling, prelude::*};

use crate::{
    game::GameSet,
    voxel::{
        VoxelBrush, VoxelChunk, VoxelChunkCoord, VoxelChunkOf, VoxelEdit, VoxelMaterialId,
        VoxelRayHit, VoxelSample, VoxelWorld, empty_voxel_mesh,
    },
};

use super::super::{
    PlaygroundCatalog, PlaygroundItem, PlaygroundItemAction, PlaygroundItemId, UsePlaygroundItem,
};

pub const VOXEL_HAND: PlaygroundItemId = PlaygroundItemId::new("voxel_hand");

const TOOL_RANGE: f32 = 64.0;
const BRUSH_RADIUS: f32 = 2.0;

#[derive(Component)]
struct EditableRock;

pub struct VoxelHandItemPlugin;

impl Plugin for VoxelHandItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Startup, spawn_editable_rock)
            .add_systems(Update, use_voxel_hand.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<PlaygroundCatalog>) {
    catalog.register(PlaygroundItem {
        id: VOXEL_HAND,
        name: "Voxel Hand",
        description: "Remove or add smooth volumetric rock.",
    });
}

fn spawn_editable_rock(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Deliberately straddle both the X=0 and Y=0 chunk planes so the initial
    // rock immediately exercises seamless neighboring chunk extraction.
    let center = Vec3::new(0.0, 4.0, -8.0);
    let radius = 6.0;
    let world_entity = commands
        .spawn((
            Name::new("Editable Voxel Rock"),
            EditableRock,
            Transform::IDENTITY,
        ))
        .id();

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.34, 0.31, 0.27),
        perceptual_roughness: 1.0,
        ..default()
    });
    let mut world = VoxelWorld::default();

    // Four logical chunks are enough for the first world-level test: the rock
    // occupies X {-1, 0}, Y {-1, 0}, Z {-1} on the 32-unit chunk grid.
    for y in -1..=0 {
        for x in -1..=0 {
            let coord = VoxelChunkCoord::new(IVec3::new(x, y, -1));
            let chunk = VoxelChunk::generate(coord.origin(), |point| {
                let distance = point.distance(center) - radius;
                VoxelSample::new(
                    distance,
                    if distance < 0.0 {
                        VoxelMaterialId::ROCK
                    } else {
                        VoxelMaterialId::VOID
                    },
                )
            });

            let chunk_entity = commands
                .spawn((
                    Name::new(format!("Voxel Chunk ({x}, {y}, -1)")),
                    VoxelChunkOf::new(world_entity, coord),
                    chunk,
                    Mesh3d(meshes.add(empty_voxel_mesh())),
                    MeshMaterial3d(material.clone()),
                    // Mesh bounds become stale after in-place remeshing. Proper
                    // chunk bounds/culling arrive with the storage hierarchy.
                    NoFrustumCulling,
                    Transform::IDENTITY,
                ))
                .id();

            commands.entity(world_entity).add_child(chunk_entity);
            assert!(
                world.insert_chunk(coord, chunk_entity).is_none(),
                "duplicate voxel chunk coordinate {coord:?}"
            );
        }
    }

    commands.entity(world_entity).insert(world);
}

fn use_voxel_hand(
    mut uses: MessageReader<UsePlaygroundItem>,
    world: Single<&VoxelWorld, With<EditableRock>>,
    mut chunks: ParamSet<(Query<&VoxelChunk>, Query<&mut VoxelChunk>)>,
) {
    for request in uses.read() {
        if request.item != VOXEL_HAND
            || (request.action != PlaygroundItemAction::PRIMARY
                && request.action != PlaygroundItemAction::SECONDARY)
        {
            continue;
        }

        let nearest = {
            let chunks = chunks.p0();
            world
                .chunk_entities()
                .filter_map(|entity| {
                    chunks
                        .get(entity)
                        .ok()?
                        .raycast(request.aim.origin, request.aim.direction, TOOL_RANGE)
                })
                .min_by(|left, right| left.distance.total_cmp(&right.distance))
        };

        let Some(VoxelRayHit { position, .. }) = nearest else {
            continue;
        };

        let direction = request.aim.direction.normalize_or_zero();
        let center = if request.action == PlaygroundItemAction::PRIMARY {
            // Bias into the surface so removing matter creates an obvious bite.
            position + direction * (BRUSH_RADIUS * 0.35)
        } else {
            // Bias outward so adding matter visibly grows the surface.
            position - direction * (BRUSH_RADIUS * 0.35)
        };
        let brush = VoxelBrush::sphere(center, BRUSH_RADIUS);

        let edit = if request.action == PlaygroundItemAction::PRIMARY {
            VoxelEdit::Remove { brush }
        } else {
            VoxelEdit::Add {
                brush,
                material: VoxelMaterialId::ROCK,
            }
        };

        // Every chunk that physically stores samples in the edit's finite
        // influence volume receives the exact same edit. This is what keeps
        // duplicated one-sample borders bit-identical across chunk seams.
        let affected = world.chunks_intersecting(edit.influence_bounds());
        let mut chunks = chunks.p1();
        for entity in affected {
            if let Ok(mut chunk) = chunks.get_mut(entity) {
                chunk.apply_edit(edit);
            }
        }
    }
}
