//! Space-Engineers-style voxel hand for the editable-rock vertical slice.

use bevy::{camera::visibility::NoFrustumCulling, prelude::*};

use crate::{
    game::GameSet,
    voxel::{
        CHUNK_SIZE, VoxelBrush, VoxelChunk, VoxelEdit, VoxelMaterialId, VoxelRayHit, VoxelSample,
        empty_voxel_mesh,
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
    let center = Vec3::new(0.0, 4.0, -5.0);
    let radius = 4.0;
    let origin = center.floor().as_ivec3() - IVec3::splat((CHUNK_SIZE / 2) as i32);

    let chunk = VoxelChunk::generate(origin, |point| {
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

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.34, 0.31, 0.27),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Name::new("Editable Voxel Rock"),
        EditableRock,
        chunk,
        Mesh3d(meshes.add(empty_voxel_mesh())),
        MeshMaterial3d(material),
        // The mesh changes in-place. M0 opts out of stale automatic bounds;
        // chunk-level bounds/culling belongs with the later storage hierarchy.
        NoFrustumCulling,
        Transform::IDENTITY,
    ));
}

fn use_voxel_hand(
    mut uses: MessageReader<UsePlaygroundItem>,
    mut chunks: ParamSet<(
        Query<(Entity, &VoxelChunk), With<EditableRock>>,
        Query<&mut VoxelChunk, With<EditableRock>>,
    )>,
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
            chunks
                .iter()
                .filter_map(|(entity, chunk)| {
                    chunk
                        .raycast(request.aim.origin, request.aim.direction, TOOL_RANGE)
                        .map(|hit| (entity, hit))
                })
                .min_by(|(_, left), (_, right)| left.distance.total_cmp(&right.distance))
        };

        let Some((entity, VoxelRayHit { position, .. })) = nearest else {
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

        if let Ok(mut chunk) = chunks.p1().get_mut(entity) {
            chunk.apply_edit(edit);
        }
    }
}
