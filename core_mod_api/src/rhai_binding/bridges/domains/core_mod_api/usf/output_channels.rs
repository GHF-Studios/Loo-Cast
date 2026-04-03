core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::output_channels,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [],
);

use crate::bevy::prelude::*;
use crate::bevy_rapier3d::prelude::{Collider, ComputedColliderShape};
use crate::config::statics::CONFIG;
use crate::usf::chunk::components::Chunk;
use crate::usf::phenomenon::{
    InteractionTriggerDefinition, OutputAudioEmitterDefinition, OutputMaterialProfileDefinition, OutputParticleEmitterDefinition,
    PhenomenonSimulationServiceDefinition,
};
use crate::usf::pos::grid::types::GridVec;
use std::collections::HashMap;

use crate::usf::chunk::realization::field::color_from_seed;
use crate::usf::chunk::realization::runtime::{ChunkRealizationCache, ChunkRealizationInstance, ChunkRealizationResolvedArtifact};

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationAudioEmitter {
    pub output: OutputAudioEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationParticleEmitter {
    pub output: OutputParticleEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationInteractionTrigger {
    pub output: InteractionTriggerDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ChunkRealizationSimulationService {
    pub output: PhenomenonSimulationServiceDefinition,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct OutputChannelRegistry {
    pub registrations_by_channel: HashMap<String, OutputChannelExecutionRegistration>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum OutputChannelPayload {
    Mesh,
    Material(OutputMaterialProfileDefinition),
    Collider,
    Audio(OutputAudioEmitterDefinition),
    Particles(OutputParticleEmitterDefinition),
    Trigger(InteractionTriggerDefinition),
    SimulationService(PhenomenonSimulationServiceDefinition),
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
    pub material_instances: u64,
    pub collider_instances: u64,
    pub audio_emitters: u64,
    pub particle_emitters: u64,
    pub interaction_triggers: u64,
    pub simulation_services: u64,
}

#[derive(Message, Reflect, Debug, Clone, PartialEq)]
pub struct ChunkRealizationChannelAppliedEvent {
    pub chunk_entity: Entity,
    pub chunk_coord: GridVec,
    pub channel_id: String,
    pub payload: OutputChannelPayload,
}

pub(crate) fn report_chunk_realization_channel_telemetry_system(
    settings: Option<Res<ChunkRealizationChannelTelemetrySettings>>,
    time: Option<Res<Time>>,
    telemetry: Option<ResMut<ChunkRealizationChannelTelemetry>>,
    mesh_query: Query<(), (With<Chunk>, With<Mesh3d>)>,
    material_query: Query<(), (With<Chunk>, With<MeshMaterial3d<StandardMaterial>>)>,
    collider_query: Query<(), (With<Chunk>, With<Collider>)>,
    audio_query: Query<(), (With<Chunk>, With<ChunkRealizationAudioEmitter>)>,
    particle_query: Query<(), (With<Chunk>, With<ChunkRealizationParticleEmitter>)>,
    trigger_query: Query<(), (With<Chunk>, With<ChunkRealizationInteractionTrigger>)>,
    simulation_query: Query<(), (With<Chunk>, With<ChunkRealizationSimulationService>)>,
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
        material_instances: material_query.iter().count() as u64,
        collider_instances: collider_query.iter().count() as u64,
        audio_emitters: audio_query.iter().count() as u64,
        particle_emitters: particle_query.iter().count() as u64,
        interaction_triggers: trigger_query.iter().count() as u64,
        simulation_services: simulation_query.iter().count() as u64,
    };
    if snapshot != *telemetry {
        *telemetry = snapshot;
    }

    warn!(
        "USF chunk output channel telemetry: mesh={} material={} collider={} audio={} particles={} trigger={} simulation_service={}",
        telemetry.mesh_instances,
        telemetry.material_instances,
        telemetry.collider_instances,
        telemetry.audio_emitters,
        telemetry.particle_emitters,
        telemetry.interaction_triggers,
        telemetry.simulation_services
    );
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputChannelAuthorityMode {
    ProjectionOnly,
    LeasedLocalAuthority,
    AuthoritativeRuntime,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct OutputChannelExecutionRegistration {
    pub channel_id: String,
    pub owner_path: String,
    pub authority_mode: OutputChannelAuthorityMode,
}
impl Default for OutputChannelRegistry {
    fn default() -> Self {
        let registrations_by_channel = [
            (
                "mesh",
                OutputChannelExecutionRegistration {
                    channel_id: "mesh".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "material",
                OutputChannelExecutionRegistration {
                    channel_id: "material".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "collider",
                OutputChannelExecutionRegistration {
                    channel_id: "collider".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::LeasedLocalAuthority,
                },
            ),
            (
                "audio",
                OutputChannelExecutionRegistration {
                    channel_id: "audio".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "particles",
                OutputChannelExecutionRegistration {
                    channel_id: "particles".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::ProjectionOnly,
                },
            ),
            (
                "trigger",
                OutputChannelExecutionRegistration {
                    channel_id: "trigger".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::LeasedLocalAuthority,
                },
            ),
            (
                "simulation_service",
                OutputChannelExecutionRegistration {
                    channel_id: "simulation_service".to_string(),
                    owner_path: "rhai_binding.bridges.domains.core_mod_api.usf.output_channels".to_string(),
                    authority_mode: OutputChannelAuthorityMode::AuthoritativeRuntime,
                },
            ),
        ]
        .into_iter()
        .map(|(id, contract)| (id.to_string(), contract))
        .collect::<HashMap<_, _>>();

        Self { registrations_by_channel }
    }
}
impl OutputChannelRegistry {
    pub fn has_registration(&self, channel: &str) -> bool {
        self.registrations_by_channel.contains_key(channel)
    }
}

pub(crate) fn apply_chunk_output_channels(
    artifact: ChunkRealizationResolvedArtifact,
    channel_registry: &OutputChannelRegistry,
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

    let mesh_material_profile = match channel_payloads.get("material") {
        Some(OutputChannelPayload::Material(profile)) => Some(*profile),
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='material' expects OutputChannelPayload::Material but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    let mesh_requested = matches!(channel_payloads.get("mesh"), Some(OutputChannelPayload::Mesh));
    let mesh_enabled = channel_registry.has_registration("mesh") && mesh_requested;
    let material_enabled = channel_registry.has_registration("material");

    let collider_requested = match channel_payloads.get("collider") {
        Some(OutputChannelPayload::Collider) => true,
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='collider' expects OutputChannelPayload::Collider but found {:?}.",
                chunk_coord, payload
            );
            false
        }
        None => false,
    };
    let collider_enabled = channel_registry.has_registration("collider") && collider_requested;
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
        let material_handle = materials.add(if material_enabled {
            match mesh_material_profile {
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
            }
        } else {
            StandardMaterial {
                base_color: color_from_seed(record.chunk_seed),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                unlit: false,
                cull_mode: None,
                ..Default::default()
            }
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
                payload: OutputChannelPayload::Collider,
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
                payload: OutputChannelPayload::Mesh,
            });
        }
        if material_enabled {
            if let Some(profile) = mesh_material_profile {
                channel_events.write(ChunkRealizationChannelAppliedEvent {
                    chunk_entity,
                    chunk_coord: chunk_coord.clone(),
                    channel_id: "material".to_string(),
                    payload: OutputChannelPayload::Material(profile),
                });
            }
        }
    } else {
        commands.entity(chunk_entity).remove::<Mesh3d>();
        commands.entity(chunk_entity).remove::<MeshMaterial3d<StandardMaterial>>();
        commands.entity(chunk_entity).remove::<Collider>();
    }

    let output_audio_emitter = match channel_payloads.get("audio") {
        Some(OutputChannelPayload::Audio(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='audio' expects OutputChannelPayload::Audio but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has_registration("audio") {
        if let Some(contract) = output_audio_emitter {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "audio".to_string(),
                payload: OutputChannelPayload::Audio(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationAudioEmitter { output: contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationAudioEmitter>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationAudioEmitter>();
    }

    let output_particle_emitter = match channel_payloads.get("particles") {
        Some(OutputChannelPayload::Particles(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='particles' expects OutputChannelPayload::Particles but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has_registration("particles") {
        if let Some(contract) = output_particle_emitter {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "particles".to_string(),
                payload: OutputChannelPayload::Particles(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationParticleEmitter { output: contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationParticleEmitter>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationParticleEmitter>();
    }

    let interaction_trigger = match channel_payloads.get("trigger") {
        Some(OutputChannelPayload::Trigger(contract)) => Some(contract.clone()),
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='trigger' expects OutputChannelPayload::Trigger but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has_registration("trigger") {
        if let Some(contract) = interaction_trigger {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "trigger".to_string(),
                payload: OutputChannelPayload::Trigger(contract.clone()),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationInteractionTrigger { output: contract });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationInteractionTrigger>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationInteractionTrigger>();
    }

    let simulation_service = match channel_payloads.get("simulation_service") {
        Some(OutputChannelPayload::SimulationService(service)) => Some(*service),
        Some(payload) => {
            warn!(
                "USF output channel payload mismatch for chunk {:?}: channel='simulation_service' expects OutputChannelPayload::SimulationService but found {:?}.",
                chunk_coord, payload
            );
            None
        }
        None => None,
    };
    if channel_registry.has_registration("simulation_service") {
        if let Some(service) = simulation_service {
            channel_events.write(ChunkRealizationChannelAppliedEvent {
                chunk_entity,
                chunk_coord: chunk_coord.clone(),
                channel_id: "simulation_service".to_string(),
                payload: OutputChannelPayload::SimulationService(service),
            });
            commands.entity(chunk_entity).insert(ChunkRealizationSimulationService { output: service });
        } else {
            commands.entity(chunk_entity).remove::<ChunkRealizationSimulationService>();
        }
    } else {
        commands.entity(chunk_entity).remove::<ChunkRealizationSimulationService>();
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
    fn default_registry_includes_flat_output_channels() {
        let registry = OutputChannelRegistry::default();
        assert!(registry.has_registration("mesh"));
        assert!(registry.has_registration("material"));
        assert!(registry.has_registration("collider"));
        assert!(registry.has_registration("audio"));
        assert!(registry.has_registration("particles"));
        assert!(registry.has_registration("trigger"));
        assert!(registry.has_registration("simulation_service"));
    }

    #[test]
    fn default_registry_has_registration_for_every_enabled_channel() {
        let registry = OutputChannelRegistry::default();
        for channel in registry.registrations_by_channel.keys() {
            assert!(
                registry.has_registration(channel),
                "missing execution registration for enabled channel '{}'",
                channel
            );
        }
    }
}
