use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::{Collider, ComputedColliderShape};

use super::chunk_manifestation::{ChunkManifestationHydrationArtifact, UsfChunkManifestationInstance, UsfChunkManifestationStore};
use super::manifestation_field::color_from_seed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChunkManifestationCapabilityPolicy {
    pub attach_meshes: bool,
}

pub(crate) fn apply_chunk_manifestation_capabilities(
    artifact: ChunkManifestationHydrationArtifact,
    capability_policy: ChunkManifestationCapabilityPolicy,
    commands: &mut Commands,
    chunk_store: &mut UsfChunkManifestationStore,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ChunkManifestationHydrationArtifact {
        chunk_entity,
        chunk_coord,
        canonical_coord,
        record,
        manifestation_material_profile,
        manifestation_collider_enabled,
        mesh,
    } = artifact;

    if !capability_policy.attach_meshes {
        commands.entity(chunk_entity).remove::<Mesh3d>();
        commands.entity(chunk_entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(chunk_entity).remove::<Collider>();
    } else if let Some(mesh) = mesh {
        let collider = if manifestation_collider_enabled {
            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::default())
        } else {
            None
        };
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(match manifestation_material_profile {
            Some(profile) => StandardMaterial {
                base_color: Color::srgba(profile.albedo_r, profile.albedo_g, profile.albedo_b, profile.alpha),
                perceptual_roughness: profile.perceptual_roughness.clamp(0.0, 1.0),
                metallic: profile.metallic.clamp(0.0, 1.0),
                emissive: Color::srgb(
                    profile.albedo_r * profile.emissive_strength,
                    profile.albedo_g * profile.emissive_strength,
                    profile.albedo_b * profile.emissive_strength,
                )
                .into(),
                unlit: false,
                cull_mode: None,
                ..Default::default()
            },
            None => StandardMaterial {
                base_color: color_from_seed(record.chunk_seed),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                unlit: false,
                cull_mode: None,
                ..Default::default()
            },
        });
        let mut entity_commands = commands.entity(chunk_entity);
        entity_commands.insert((Mesh3d(mesh_handle), MeshMaterial3d(material_handle), Visibility::Visible));
        if !manifestation_collider_enabled {
            entity_commands.remove::<Collider>();
        } else if let Some(collider) = collider {
            entity_commands.insert(collider);
        } else {
            warn!(
                "USF runtime collider build failed for chunk {:?}; mesh will render without collision.",
                chunk_coord
            );
            entity_commands.remove::<Collider>();
        }
    } else {
        commands.entity(chunk_entity).remove::<Mesh3d>();
        commands.entity(chunk_entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(chunk_entity).remove::<Collider>();
    }

    commands.entity(chunk_entity).insert(UsfChunkManifestationInstance {
        chunk_seed: record.chunk_seed,
        sample_step: record.sample_step,
    });

    chunk_store.records.insert(canonical_coord, record);
}
