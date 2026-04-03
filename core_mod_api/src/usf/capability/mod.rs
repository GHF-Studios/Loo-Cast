use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);
impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into().trim().to_ascii_lowercase())
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAuthorityMode {
    ProjectionOnly,
    LeasedLocalAuthority,
    AuthoritativeRuntime,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct CapabilityExecutionContract {
    pub id: CapabilityId,
    pub owner_path: String,
    pub authority_mode: CapabilityAuthorityMode,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfCapabilityGraph {
    pub world_capabilities: Vec<CapabilityId>,
    pub presentation_capabilities: Vec<CapabilityId>,
    pub simulation_capabilities: Vec<CapabilityId>,
    pub interaction_capabilities: Vec<CapabilityId>,
    pub contracts_by_capability: HashMap<CapabilityId, CapabilityExecutionContract>,
}
impl Default for UsfCapabilityGraph {
    fn default() -> Self {
        let world_capabilities = vec![CapabilityId::new("world.chunk_manifestation.derived_cache")];
        let presentation_capabilities = vec![
            CapabilityId::new("presentation.chunk_manifestation.instance_render"),
            CapabilityId::new("presentation.chunk_manifestation.instance_audio"),
            CapabilityId::new("presentation.chunk_manifestation.instance_particles"),
        ];
        let simulation_capabilities = vec![CapabilityId::new("simulation.chunk_manifestation.instance_collider")];
        let interaction_capabilities = vec![CapabilityId::new("interaction.chunk_manifestation.instance_trigger")];

        let mut contracts_by_capability = HashMap::new();
        contracts_by_capability.insert(
            CapabilityId::new("world.chunk_manifestation.derived_cache"),
            CapabilityExecutionContract {
                id: CapabilityId::new("world.chunk_manifestation.derived_cache"),
                owner_path: "usf.runtime.manifestation.runtime".to_string(),
                authority_mode: CapabilityAuthorityMode::ProjectionOnly,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("presentation.chunk_manifestation.instance_render"),
            CapabilityExecutionContract {
                id: CapabilityId::new("presentation.chunk_manifestation.instance_render"),
                owner_path: "usf.runtime.capability.manifestation".to_string(),
                authority_mode: CapabilityAuthorityMode::ProjectionOnly,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("simulation.chunk_manifestation.instance_collider"),
            CapabilityExecutionContract {
                id: CapabilityId::new("simulation.chunk_manifestation.instance_collider"),
                owner_path: "usf.runtime.capability.manifestation".to_string(),
                authority_mode: CapabilityAuthorityMode::LeasedLocalAuthority,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("presentation.chunk_manifestation.instance_audio"),
            CapabilityExecutionContract {
                id: CapabilityId::new("presentation.chunk_manifestation.instance_audio"),
                owner_path: "usf.runtime.capability.manifestation".to_string(),
                authority_mode: CapabilityAuthorityMode::ProjectionOnly,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("presentation.chunk_manifestation.instance_particles"),
            CapabilityExecutionContract {
                id: CapabilityId::new("presentation.chunk_manifestation.instance_particles"),
                owner_path: "usf.runtime.capability.manifestation".to_string(),
                authority_mode: CapabilityAuthorityMode::ProjectionOnly,
            },
        );
        contracts_by_capability.insert(
            CapabilityId::new("interaction.chunk_manifestation.instance_trigger"),
            CapabilityExecutionContract {
                id: CapabilityId::new("interaction.chunk_manifestation.instance_trigger"),
                owner_path: "usf.runtime.capability.manifestation".to_string(),
                authority_mode: CapabilityAuthorityMode::LeasedLocalAuthority,
            },
        );

        Self {
            world_capabilities,
            presentation_capabilities,
            simulation_capabilities,
            interaction_capabilities,
            contracts_by_capability,
        }
    }
}

fn validate_capability_graph_system(graph: Res<UsfCapabilityGraph>) {
    if graph.world_capabilities.is_empty() {
        panic!("USF capability graph validation failed: world.* capability bucket must not be empty");
    }
    if graph.presentation_capabilities.is_empty() {
        panic!("USF capability graph validation failed: presentation.* capability bucket must not be empty");
    }
    if graph.simulation_capabilities.is_empty() {
        panic!("USF capability graph validation failed: simulation.* capability bucket must not be empty");
    }
    if graph.interaction_capabilities.is_empty() {
        panic!("USF capability graph validation failed: interaction.* capability bucket must not be empty");
    }

    for capability in graph
        .world_capabilities
        .iter()
        .chain(graph.presentation_capabilities.iter())
        .chain(graph.simulation_capabilities.iter())
        .chain(graph.interaction_capabilities.iter())
    {
        if !graph.contracts_by_capability.contains_key(capability) {
            panic!(
                "USF capability graph validation failed: missing execution contract for capability '{}'",
                capability.0
            );
        }
    }
}

pub(crate) struct CapabilityPlugin;
impl Plugin for CapabilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfCapabilityGraph>()
            .add_systems(Startup, validate_capability_graph_system.in_set(AppSet::Diagnostics))
            .register_type::<CapabilityId>()
            .register_type::<CapabilityAuthorityMode>()
            .register_type::<CapabilityExecutionContract>()
            .register_type::<UsfCapabilityGraph>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contains_capability(capabilities: &[CapabilityId], expected: &str) -> bool {
        let expected = CapabilityId::new(expected);
        capabilities.contains(&expected)
    }

    #[test]
    fn default_capability_graph_includes_extended_manifestation_families() {
        let graph = UsfCapabilityGraph::default();
        assert!(contains_capability(
            &graph.presentation_capabilities,
            "presentation.chunk_manifestation.instance_render"
        ));
        assert!(contains_capability(
            &graph.presentation_capabilities,
            "presentation.chunk_manifestation.instance_audio"
        ));
        assert!(contains_capability(
            &graph.presentation_capabilities,
            "presentation.chunk_manifestation.instance_particles"
        ));
        assert!(contains_capability(
            &graph.simulation_capabilities,
            "simulation.chunk_manifestation.instance_collider"
        ));
        assert!(contains_capability(
            &graph.interaction_capabilities,
            "interaction.chunk_manifestation.instance_trigger"
        ));
    }

    #[test]
    fn default_capability_graph_has_contract_for_every_declared_capability() {
        let graph = UsfCapabilityGraph::default();
        for capability in graph
            .world_capabilities
            .iter()
            .chain(graph.presentation_capabilities.iter())
            .chain(graph.simulation_capabilities.iter())
            .chain(graph.interaction_capabilities.iter())
        {
            assert!(
                graph.contracts_by_capability.contains_key(capability),
                "missing contract for capability '{}'",
                capability.0
            );
        }
    }
}
