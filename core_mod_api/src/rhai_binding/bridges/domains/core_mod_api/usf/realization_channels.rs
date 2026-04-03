use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use crate::config::statics::CONFIG;
use crate::usf::chunk::components::Chunk;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::phenomenon::{
    InteractionTriggerDefinition, RealizationAudioEmitterDefinition, RealizationMaterialProfileDefinition, RealizationParticleEmitterDefinition,
};
use std::collections::{HashMap, HashSet};

use crate::usf::chunk::realization::field::color_from_seed;
use crate::usf::chunk::realization::runtime::{
    ChunkRealizationCache, ChunkRealizationInstance, ChunkRealizationResolvedArtifact,
};

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationAudioEmitter {
    pub contract: RealizationAudioEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationParticleEmitter {
    pub contract: RealizationParticleEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationInteractionTrigger {
    pub contract: InteractionTriggerDefinition,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct RealizationChannelRegistry {
    pub enabled_channels: HashSet<String>,
    pub contracts_by_channel: HashMap<String, RealizationChannelExecutionContract>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum RealizationChannelPayload {
    Mesh {
        material_profile: Option<RealizationMaterialProfileDefinition>,
    },
    Collider,
    Audio(RealizationAudioEmitterDefinition),
    Particles(RealizationParticleEmitterDefinition),
    Trigger(InteractionTriggerDefinition),
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq)]
#[reflect(Resource)]
pub struct ChunkRealizationChannelTelemetrySettings {
    pub enabled: bool,
    pub log_interval_seconds: f32,
}
impl Default for ChunkRealizationChannelTelemetrySettings {
    fn default() -> Self {
        let log_interval_seconds = CONFIG().get::<f32>("usf/chunk/realization/channel_telemetry_log_interval_seconds");
        if !log_interval_seconds.is_finite() || log_interval_seconds <= 0.0 {
            panic!(
                "USF chunk realization telemetry config is invalid: channel_telemetry_log_interval_seconds must be finite > 0, got {}.",
                log_interval_seconds
            );
        }
        Self {
            enabled: CONFIG().get::<bool>("usf/chunk/realization/channel_telemetry_enabled"),
            log_interval_seconds,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Resource)]
pub struct ChunkRealizationChannelTelemetry {
    pub mesh_instances: u64,
    pub collider_instances: u64,
    pub audio_emitters: u64,
    pub particle_emitters: u64,
    pub interaction_triggers: u64,
}

#[derive(Message, Reflect, Debug, Clone, PartialEq)]
pub struct ChunkRealizationChannelAppliedEvent {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub channel_id: String,
    pub payload: RealizationChannelPayload,
}

pub(crate) fn report_chunk_realization_channel_telemetry_system(
    settings: Option<Res<ChunkRealizationChannelTelemetrySettings>>,
    time: Option<Res<Time>>,
    telemetry: Option<ResMut<ChunkRealizationChannelTelemetry>>,
    mesh_query: Query<(), (With<Chunk>, With<Mesh3d>)>,
    collider_query: Query<(), (With<Chunk>, With<Collider>)>,
    audio_query: Query<(), (With<Chunk>, With<ChunkRealizationAudioEmitter>)>,
    particle_query: Query<(), (With<Chunk>, With<ChunkRealizationParticleEmitter>)>,
    trigger_query: Query<(), (With<Chunk>, With<ChunkRealizationInteractionTrigger>)>,
    mut log_timer: Local<Option<Timer>>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.enabled {
        return;
    }
    let Some(time) = time else {
        return;
    };
    let Some(mut telemetry) = telemetry else {
        return;
    };

    let timer = log_timer.get_or_insert_with(|| Timer::from_seconds(settings.log_interval_seconds, TimerMode::Repeating));
    if settings.is_changed() {
        timer.set_duration(std::time::Duration::from_secs_f32(settings.log_interval_seconds));
        timer.reset();
    }
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let snapshot = ChunkRealizationChannelTelemetry {
        mesh_instances: mesh_query.iter().count() as u64,
        collider_instances: collider_query.iter().count() as u64,
        audio_emitters: audio_query.iter().count() as u64,
        particle_emitters: particle_query.iter().count() as u64,
        interaction_triggers: trigger_query.iter().count() as u64,
    };
    if snapshot != *telemetry {
        *telemetry = snapshot;
    }

    warn!(
        "USF chunk realization channel telemetry: mesh={} collider={} audio={} particles={} trigger={}",
        telemetry.mesh_instances,
        telemetry.collider_instances,
        telemetry.audio_emitters,
        telemetry.particle_emitters,
        telemetry.interaction_triggers
    );
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealizationChannelAuthorityMode {
    ProjectionOnly,
    LeasedLocalAuthority,
    AuthoritativeRuntime,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct RealizationChannelExecutionContract {
    pub channel_id: String,
    pub owner_path: String,
    pub authority_mode: RealizationChannelAuthorityMode,
}
impl Default for RealizationChannelRegistry {
    fn default() -> Self {
        let enabled_channels = ["mesh", "collider", "audio", "particles", "trigger"]
            .into_iter()
            .map(|value| value.to_string())
            .collect::<HashSet<_>>();
        let contracts_by_channel = [
            (
                "mesh",
                RealizationChannelExecutionContract {
                    channel_id: "mesh".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.realization_channels".to_string(),
                    authority_mode: RealizationChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "collider",
                RealizationChannelExecutionContract {
                    channel_id: "collider".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.realization_channels".to_string(),
                    authority_mode: RealizationChannelAuthorityMode::LeasedLocalAuthority,
                },
            ),
            (
                "audio",
                RealizationChannelExecutionContract {
                    channel_id: "audio".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.realization_channels".to_string(),
                    authority_mode: RealizationChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "particles",
                RealizationChannelExecutionContract {
                    channel_id: "particles".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.realization_channels".to_string(),
                    authority_mode: RealizationChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "trigger",
                RealizationChannelExecutionContract {
                    channel_id: "trigger".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.realization_channels".to_string(),
                    authority_mode: RealizationChannelAuthorityMode::LeasedLocalAuthority,
                },
            ),
        ]
        .into_iter()
        .map(|(id, contract)| (id.to_string(), contract))
        .collect::<HashMap<_, _>>();

        Self {
            enabled_channels,
            contracts_by_channel,
        }
    }
}
impl RealizationChannelRegistry {
    pub fn has(&self, channel: &str) -> bool {
        self.enabled_channels.contains(channel)
    }

    pub fn has_contract(&self, channel: &str) -> bool {
        self.contracts_by_channel.contains_key(channel)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ChunkRealizationChannelPolicy {
    pub attach_meshes: bool,
}

pub(crate) fn apply_chunk_realization_channels(
    artifact: ChunkRealizationResolvedArtifact,
    channel_policy: ChunkRealizationChannelPolicy,
    channel_registry: &RealizationChannelRegistry,
    channel_events: &mut MessageWriter<ChunkRealizationChannelAppliedEvent>,
    commands: &mut Commands,
    chunk_store: &mut ChunkRealizationCache,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ChunkRealizationResolvedArtifact {
        chunk_entity,
        chunk_coord,
        canonical_coord,
        record,
        channel_payloads,
        mesh,
    } = artifact;

    let mesh_material_profile = match channel_payloads.get("mesh") {
        Some(RealizationChannelPayload::Mesh { material_profile }) => *material_profile,
        Some(payload) => {
            warn!(
                "USF realization channel payload mismatch for chunk {:?}: channel='mesh' expects RealizationChannelPayload::Mesh but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    let mesh_requested = matches!(
        channel_payloads.get("mesh"),
        Some(RealizationChannelPayload::Mesh { .. })
    );
    let mesh_enabled = channel_policy.attach_meshes && channel_registry.has("mesh") && mesh_requested;

    let collider_requested = match channel_payloads.get("collider") {
        Some(RealizationChannelPayload::Collider) => true,
        Some(payload) => {
            warn!(
                "USF realization channel payload mismatch for chunk {:?}: channel='collider' expects RealizationChannelPayload::Collider but found {:?}.",
                chunk_coord, payload
            );
            false
        }
        None => false,
    };
    let collider_enabled = channel_registry.has("collider") && collider_requested;
    if !mesh_enabled {
        commands.entity(chunk_entity).remove::<Mesh3d>();
        commands.entity(chunk_entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(chunk_entity).remove::<Collider>();
    } else if let Some(mesh) = mesh {
        let collider = if collider_enabled {
            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::default())
        } else {
            None
        };
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(match mesh_material_profile {
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
        if !collider_enabled {
            entity_commands.remove::<Collider>();
        } else if let Some(collider) = collider {
            entity_commands.insert(collider);
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "collider".to_string(),
                payload: RealizationChannelPayload::Collider,
            });
        } else {
            warn!(
                "USF runtime collider build failed for chunk {:?}; mesh will render without collision.",
                chunk_coord
            );
            entity_commands.remove::<Collider>();
        }
        if mesh_requested {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "mesh".to_string(),
                payload: RealizationChannelPayload::Mesh {
                    material_profile: mesh_material_profile,
                },
            });
        }
    } else {
        commands.entity(chunk_entity).remove::<Mesh3d>();
        commands.entity(chunk_entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(chunk_entity).remove::<Collider>();
    }

    let realization_audio_emitter = match channel_payloads.get("audio") {
        Some(RealizationChannelPayload::Audio(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF realization channel payload mismatch for chunk {:?}: channel='audio' expects RealizationChannelPayload::Audio but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has("audio") {
        if let Some(contract) = realization_audio_emitter {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "audio".to_string(),
                payload: RealizationChannelPayload::Audio(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationAudioEmitter { contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationAudioEmitter>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationAudioEmitter>();
    }

    let realization_particle_emitter = match channel_payloads.get("particles") {
        Some(RealizationChannelPayload::Particles(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF realization channel payload mismatch for chunk {:?}: channel='particles' expects RealizationChannelPayload::Particles but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has("particles") {
        if let Some(contract) = realization_particle_emitter {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "particles".to_string(),
                payload: RealizationChannelPayload::Particles(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationParticleEmitter { contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationParticleEmitter>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationParticleEmitter>();
    }

    let interaction_trigger = match channel_payloads.get("trigger") {
        Some(RealizationChannelPayload::Trigger(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF realization channel payload mismatch for chunk {:?}: channel='trigger' expects RealizationChannelPayload::Trigger but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has("trigger") {
        if let Some(contract) = interaction_trigger {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "trigger".to_string(),
                payload: RealizationChannelPayload::Trigger(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationInteractionTrigger { contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationInteractionTrigger>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationInteractionTrigger>();
    }

    commands.entity(chunk_entity).insert(ChunkRealizationInstance {
        chunk_seed: record.chunk_seed,
        sample_step: record.sample_step,
    });

    chunk_store.records.insert(canonical_coord, record);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_includes_flat_realization_channels() {
        let registry = RealizationChannelRegistry::default();
        assert!(registry.has("mesh"));
        assert!(registry.has("collider"));
        assert!(registry.has("audio"));
        assert!(registry.has("particles"));
        assert!(registry.has("trigger"));
    }

    #[test]
    fn default_registry_has_contract_for_every_enabled_channel() {
        let registry = RealizationChannelRegistry::default();
        for channel in &registry.enabled_channels {
            assert!(
                registry.has_contract(channel),
                "missing execution contract for enabled channel '{}'",
                channel
            );
        }
    }
}
