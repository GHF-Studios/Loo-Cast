use crate::bevy::prelude::*;

use crate::usf::phenomenon::generator::{
    BuildStateInput, MeshWindowInput, PhenomenonChildPlan, PhenomenonGenerator, PhenomenonMeshWindow, PhenomenonStateSnapshot, PlanChildrenInput,
};
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;
#[cfg(test)]
use crate::usf::phenomenon::types::{PhenomenonId, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};

#[derive(Debug, Clone, Copy)]
pub struct LayerEchoGenerator {
    pub channel_decay: f32,
    pub echo_gain: f32,
    pub max_branching: u32,
}

impl Default for LayerEchoGenerator {
    fn default() -> Self {
        Self {
            channel_decay: 0.72,
            echo_gain: 0.41,
            max_branching: 8,
        }
    }
}

impl PhenomenonGenerator for LayerEchoGenerator {
    fn build_state(&self, input: BuildStateInput<'_>) -> PhenomenonStateSnapshot {
        let seed = input.key.deterministic_seed();
        let lineage_depth = input.parent_state.map_or(0, |parent| parent.lineage_depth.saturating_add(1));
        let base = seeded_channels(seed.0);

        let channels = if let Some(parent) = input.parent_state {
            // Child state intentionally depends on parent snapshot to encode lineage.
            let cell = input.key.local_cell().as_ivec3().as_vec3();
            let lineage_echo = Vec4::new(cell.x, cell.y, cell.z, input.key.local_index as f32) * 0.013;
            parent.channels * self.channel_decay + base * self.echo_gain + lineage_echo
        } else {
            base
        };

        PhenomenonStateSnapshot {
            seed,
            lineage_depth,
            channels,
        }
    }

    fn sample_density(&self, state: &PhenomenonStateSnapshot, point_local: Vec3) -> f32 {
        let probe = Vec4::new(point_local.x, point_local.y, point_local.z, 1.0);
        let dot = state.channels.dot(probe);
        let phase = (state.seed.0 as f32 * 0.000_001).fract() * std::f32::consts::TAU;
        (dot + phase.sin() * 0.15).tanh()
    }

    fn plan_children(&self, input: PlanChildrenInput<'_>) -> Vec<PhenomenonChildPlan> {
        if input.key.scale == Scale::MIN {
            return Vec::new();
        }
        let next_scale = input.key.scale.zoomed_in();

        let offsets = [
            LocalCell3::new_local(-1, -1, -1),
            LocalCell3::new_local(-1, -1, 1),
            LocalCell3::new_local(-1, 1, -1),
            LocalCell3::new_local(-1, 1, 1),
            LocalCell3::new_local(1, -1, -1),
            LocalCell3::new_local(1, -1, 1),
            LocalCell3::new_local(1, 1, -1),
            LocalCell3::new_local(1, 1, 1),
        ];

        let requested = input.max_children.min(self.max_branching).min(offsets.len() as u32) as usize;
        offsets
            .iter()
            .take(requested)
            .enumerate()
            .map(|(i, offset)| PhenomenonChildPlan {
                local_index: i as u32,
                local_cell: *offset,
                scale: next_scale,
            })
            .collect()
    }

    fn mesh_window(&self, input: MeshWindowInput<'_>) -> PhenomenonMeshWindow {
        let resolution = input.target_resolution.clamp(8, 96);
        PhenomenonMeshWindow {
            lattice_min: IVec3::splat(-16),
            lattice_max: IVec3::splat(16),
            lattice_resolution: UVec3::splat(resolution),
        }
    }
}

fn seeded_channels(seed: u64) -> Vec4 {
    Vec4::new(
        seed_to_unit(seed ^ 0x243f_6a88_85a3_08d3),
        seed_to_unit(seed ^ 0x1319_8a2e_0370_7344),
        seed_to_unit(seed ^ 0xa409_3822_299f_31d0),
        seed_to_unit(seed ^ 0x082e_fa98_ec4e_6c89),
    )
}

fn seed_to_unit(seed: u64) -> f32 {
    let mixed = splitmix64(seed);
    let unit = (mixed as f64) / (u64::MAX as f64);
    (unit as f32) * 2.0 - 1.0
}

fn splitmix64(mut state: u64) -> u64 {
    state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root_key() -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(100),
            scale: Scale::MAX,
            lineage: PhenomenonLineage::root(),
            parent: None,
            local_index: 0,
        }
    }

    fn child_key(parent_key: &PhenomenonNodeKey, child: &PhenomenonChildPlan) -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: parent_key.phenomenon_id,
            scale: child.scale,
            lineage: parent_key.lineage.pushed(child.local_cell),
            parent: Some(parent_key.deterministic_seed()),
            local_index: child.local_index,
        }
    }

    #[test]
    fn layer_echo_child_state_depends_on_parent_snapshot() {
        let generator = LayerEchoGenerator::default();
        let parent_key = root_key();
        let parent = generator.build_state(BuildStateInput {
            key: parent_key.clone(),
            parent_state: None,
        });
        let child_plan = generator.plan_children(PlanChildrenInput {
            key: parent_key.clone(),
            state: &parent,
            max_children: 1,
        })[0];
        let child_key = child_key(&parent_key, &child_plan);

        let with_parent = generator.build_state(BuildStateInput {
            key: child_key.clone(),
            parent_state: Some(&parent),
        });
        let without_parent = generator.build_state(BuildStateInput {
            key: child_key,
            parent_state: None,
        });

        assert_ne!(with_parent.channels, without_parent.channels);
        assert_eq!(with_parent.seed, without_parent.seed);
    }

    #[test]
    fn layer_echo_regenerates_deterministically_for_five_scales() {
        fn regenerate(generator: LayerEchoGenerator) -> Vec<(PhenomenonNodeSeed, Vec4)> {
            let mut out = Vec::new();
            let mut key = root_key();
            let mut state = generator.build_state(BuildStateInput {
                key: key.clone(),
                parent_state: None,
            });
            out.push((state.seed, state.channels));

            for _ in 0..4 {
                let plan = generator.plan_children(PlanChildrenInput {
                    key: key.clone(),
                    state: &state,
                    max_children: 1,
                });
                let Some(first_child) = plan.first().copied() else {
                    break;
                };
                let next_key = child_key(&key, &first_child);
                let next_state = generator.build_state(BuildStateInput {
                    key: next_key.clone(),
                    parent_state: Some(&state),
                });
                out.push((next_state.seed, next_state.channels));
                key = next_key;
                state = next_state;
            }

            out
        }

        let run_a = regenerate(LayerEchoGenerator::default());
        let run_b = regenerate(LayerEchoGenerator::default());

        assert!(run_a.len() >= 5, "expected at least 5 deterministic scales, got {}", run_a.len());
        assert_eq!(run_a, run_b);
    }

    #[test]
    fn layer_echo_regenerates_deterministically_across_full_71_scale_span() {
        fn regenerate_full_span(generator: LayerEchoGenerator) -> Vec<(PhenomenonNodeSeed, Vec4)> {
            let mut out = Vec::new();
            let mut key = root_key();
            let mut state = generator.build_state(BuildStateInput {
                key: key.clone(),
                parent_state: None,
            });
            out.push((state.seed, state.channels));

            for _ in 0..(Scale::SCALE_LEVEL_COUNT as usize - 1) {
                let plan = generator.plan_children(PlanChildrenInput {
                    key: key.clone(),
                    state: &state,
                    max_children: 1,
                });
                let Some(first_child) = plan.first().copied() else {
                    break;
                };
                let next_key = child_key(&key, &first_child);
                let next_state = generator.build_state(BuildStateInput {
                    key: next_key.clone(),
                    parent_state: Some(&state),
                });
                out.push((next_state.seed, next_state.channels));
                key = next_key;
                state = next_state;
            }

            out
        }

        let run_a = regenerate_full_span(LayerEchoGenerator::default());
        let run_b = regenerate_full_span(LayerEchoGenerator::default());
        assert_eq!(run_a.len(), Scale::SCALE_LEVEL_COUNT as usize);
        assert_eq!(run_a, run_b);
    }

    #[test]
    fn layer_echo_parent_child_reproducibility_from_snapshot() {
        let generator = LayerEchoGenerator::default();
        let parent_key = root_key();
        let parent = generator.build_state(BuildStateInput {
            key: parent_key.clone(),
            parent_state: None,
        });
        let child_plan = generator.plan_children(PlanChildrenInput {
            key: parent_key.clone(),
            state: &parent,
            max_children: 1,
        })[0];
        let child_key = child_key(&parent_key, &child_plan);

        let first = generator.build_state(BuildStateInput {
            key: child_key.clone(),
            parent_state: Some(&parent),
        });
        let second = generator.build_state(BuildStateInput {
            key: child_key,
            parent_state: Some(&parent),
        });
        assert_eq!(first, second);
    }
}
