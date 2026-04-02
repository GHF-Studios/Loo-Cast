use super::{AdaptiveSubstrateOctreeNode, ModelProjectionContribution, SubstrateLeafContainer, SubstrateRefinementState, SubstrateTransitionDecision};
use crate::bevy::prelude::*;
use crate::usf::phenomenon::PhenomenonModelTopology;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::GridXyz;

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq)]
#[reflect(Resource)]
pub(crate) struct SubstrateTransitionPolicy {
    pub refine_energy_threshold: f32,
    pub refine_instability_threshold: f32,
    pub refine_gradient_threshold: f32,
    pub coarsen_energy_threshold: f32,
    pub coarsen_instability_threshold: f32,
    pub coarsen_gradient_threshold: f32,
    pub delegate_partition_value_threshold: f32,
    pub palette_instability_threshold: f32,
    pub gradient_threshold: f32,
}
impl Default for SubstrateTransitionPolicy {
    fn default() -> Self {
        Self {
            refine_energy_threshold: 0.62,
            refine_instability_threshold: 0.50,
            refine_gradient_threshold: 0.44,
            coarsen_energy_threshold: 0.22,
            coarsen_instability_threshold: 0.16,
            coarsen_gradient_threshold: 0.18,
            delegate_partition_value_threshold: 0.65,
            palette_instability_threshold: 0.33,
            gradient_threshold: 0.24,
        }
    }
}
impl SubstrateTransitionPolicy {
    pub fn should_refine(self, state: SubstrateRefinementState) -> bool {
        state.energy > self.refine_energy_threshold || state.instability > self.refine_instability_threshold || state.gradient > self.refine_gradient_threshold
    }

    pub fn should_coarsen(self, state: SubstrateRefinementState) -> bool {
        state.energy < self.coarsen_energy_threshold
            && state.instability < self.coarsen_instability_threshold
            && state.gradient < self.coarsen_gradient_threshold
    }
}

pub(super) fn compute_refinement_state(
    metric_vector: &[f32],
    contributions: &[ModelProjectionContribution],
    parent_metrics: Option<&[f32]>,
) -> SubstrateRefinementState {
    let contribution_count = contributions.len().max(1) as f32;
    let metric_count = metric_vector.len().max(1) as f32;
    let energy = (contributions.iter().map(|contribution| contribution.value.abs()).sum::<f32>() / contribution_count).clamp(0.0, 1.0);
    let mean = metric_vector.iter().copied().sum::<f32>() / metric_count;
    let instability = (metric_vector.iter().map(|value| (value - mean).abs()).sum::<f32>() / metric_count).clamp(0.0, 1.0);
    let gradient = parent_metrics
        .map(|parent_metrics| {
            let sample_count = metric_vector.len().min(parent_metrics.len()).max(1) as f32;
            metric_vector
                .iter()
                .zip(parent_metrics.iter())
                .map(|(child, parent)| (child - parent).abs())
                .sum::<f32>()
                / sample_count
        })
        .unwrap_or_else(|| instability * 0.5)
        .clamp(0.0, 1.0);
    SubstrateRefinementState { energy, instability, gradient }
}

pub(super) fn choose_leaf_representation(
    canonical_coord: &GridVec,
    metric_vector: &[f32],
    contributions: &[ModelProjectionContribution],
    refinement_state: SubstrateRefinementState,
    transition_policy: SubstrateTransitionPolicy,
) -> SubstrateLeafContainer {
    if let Some(delegated) = contributions.iter().find(|contribution| {
        contribution.topology == PhenomenonModelTopology::PartitionedByChunk && contribution.value >= transition_policy.delegate_partition_value_threshold
    }) {
        return SubstrateLeafContainer::DelegatedToPhenomenon {
            phenomenon_id: delegated.phenomenon_id,
            model_id: delegated.model_id.clone(),
            scale_index: canonical_coord.scale.index_from_top(),
        };
    }

    if transition_policy.should_refine(refinement_state) {
        return SubstrateLeafContainer::DenseBrick {
            values: build_dense_leaf_values(metric_vector, canonical_coord),
            axis_resolution: 4,
        };
    }

    if refinement_state.instability > transition_policy.palette_instability_threshold {
        let mut palette = metric_vector
            .iter()
            .copied()
            .flat_map(|value| [value.clamp(0.0, 1.0), (value * 0.6 + 0.2).clamp(0.0, 1.0)])
            .collect::<Vec<_>>();
        palette.sort_by(|a, b| a.total_cmp(b));
        palette.dedup_by(|a, b| (*a - *b).abs() <= f32::EPSILON);
        let palette = palette.into_iter().take(16).collect::<Vec<_>>();
        let indices = (0..64).map(|index| (index % palette.len().max(1)) as u8).collect::<Vec<_>>();
        return SubstrateLeafContainer::PaletteBrick {
            palette,
            indices,
            axis_resolution: 4,
        };
    }

    if refinement_state.gradient > transition_policy.gradient_threshold {
        return SubstrateLeafContainer::Gradient {
            origin: [0.0, 0.0, 0.0],
            gradient: [refinement_state.gradient, refinement_state.gradient * 0.5, refinement_state.gradient * 0.25],
            base: metric_vector.first().copied().unwrap_or(0.5),
        };
    }

    if transition_policy.should_coarsen(refinement_state) {
        let mean = metric_vector.iter().copied().sum::<f32>() / metric_vector.len().max(1) as f32;
        return SubstrateLeafContainer::Uniform { value: mean };
    }

    let mean = metric_vector.iter().copied().sum::<f32>() / metric_vector.len().max(1) as f32;
    let variance = metric_vector.iter().map(|value| (value - mean) * (value - mean)).sum::<f32>() / metric_vector.len().max(1) as f32;
    SubstrateLeafContainer::Statistical {
        mean,
        variance,
        min: metric_vector.iter().copied().fold(f32::INFINITY, f32::min),
        max: metric_vector.iter().copied().fold(f32::NEG_INFINITY, f32::max),
    }
}

pub(super) fn build_octree_from_leaf(
    refinement_state: SubstrateRefinementState,
    base_leaf: &SubstrateLeafContainer,
    metric_vector: &[f32],
    contributions: &[ModelProjectionContribution],
    transition_decision: SubstrateTransitionDecision,
    scale_index: u8,
) -> AdaptiveSubstrateOctreeNode {
    if !transition_decision.refine {
        let leaf = transition_decision.target_leaf.unwrap_or_else(|| base_leaf.clone());
        return AdaptiveSubstrateOctreeNode::Leaf { state: refinement_state, leaf };
    }

    let mut children = Vec::with_capacity(8);
    for child_index in 0..8_u8 {
        let transitioned = transition_leaf_representation(base_leaf, child_index, refinement_state, metric_vector, contributions, scale_index);
        children.push(AdaptiveSubstrateOctreeNode::Leaf {
            state: refinement_state,
            leaf: transitioned,
        });
    }
    AdaptiveSubstrateOctreeNode::Branch {
        state: refinement_state,
        children,
    }
}

pub(super) fn derive_transition_decision_from_state(
    refinement_state: SubstrateRefinementState,
    base_leaf: &SubstrateLeafContainer,
    transition_policy: SubstrateTransitionPolicy,
) -> SubstrateTransitionDecision {
    if transition_policy.should_refine(refinement_state) {
        return SubstrateTransitionDecision {
            refine: true,
            coarsen: false,
            target_leaf: None,
        };
    }

    if transition_policy.should_coarsen(refinement_state) {
        let target_leaf = match base_leaf {
            SubstrateLeafContainer::Uniform { .. } => base_leaf.clone(),
            _ => SubstrateLeafContainer::Uniform {
                value: refinement_state.energy.clamp(0.0, 1.0),
            },
        };
        return SubstrateTransitionDecision {
            refine: false,
            coarsen: true,
            target_leaf: Some(target_leaf),
        };
    }

    SubstrateTransitionDecision {
        refine: false,
        coarsen: false,
        target_leaf: Some(base_leaf.clone()),
    }
}

pub(super) fn leaf_kind_tag(leaf: &SubstrateLeafContainer) -> &'static str {
    match leaf {
        SubstrateLeafContainer::Uniform { .. } => "uniform",
        SubstrateLeafContainer::DenseBrick { .. } => "dense_brick",
        SubstrateLeafContainer::PaletteBrick { .. } => "palette_brick",
        SubstrateLeafContainer::Gradient { .. } => "gradient",
        SubstrateLeafContainer::Statistical { .. } => "statistical",
        SubstrateLeafContainer::Heightfield { .. } => "heightfield",
        SubstrateLeafContainer::DelegatedToPhenomenon { .. } => "delegated_to_phenomenon",
    }
}

fn transition_leaf_representation(
    base_leaf: &SubstrateLeafContainer,
    child_index: u8,
    refinement_state: SubstrateRefinementState,
    metric_vector: &[f32],
    contributions: &[ModelProjectionContribution],
    scale_index: u8,
) -> SubstrateLeafContainer {
    let child_mix = (child_index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let transition_selector = ((refinement_state.energy * 1000.0) as u64 ^ (refinement_state.instability * 2000.0) as u64 ^ child_mix) % 3;
    match transition_selector {
        0 => SubstrateLeafContainer::DenseBrick {
            values: build_dense_leaf_values(metric_vector, &GridVec::new_root(GridXyz::new_local(0, 0, 0))),
            axis_resolution: 4,
        },
        1 => {
            let heights = metric_vector
                .iter()
                .enumerate()
                .map(|(index, value)| (value + (index as f32 * 0.03)).clamp(0.0, 1.0))
                .collect::<Vec<_>>();
            let min = heights.iter().copied().fold(f32::INFINITY, f32::min);
            let max = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            SubstrateLeafContainer::Heightfield {
                heights,
                axis_resolution: 4,
                min,
                max,
            }
        }
        _ => {
            if let Some(contribution) = contributions
                .iter()
                .find(|contribution| contribution.topology == PhenomenonModelTopology::PartitionedByChunk)
            {
                return SubstrateLeafContainer::DelegatedToPhenomenon {
                    phenomenon_id: contribution.phenomenon_id,
                    model_id: contribution.model_id.clone(),
                    scale_index,
                };
            }
            base_leaf.clone()
        }
    }
}

fn build_dense_leaf_values(metric_vector: &[f32], canonical_coord: &GridVec) -> Vec<f32> {
    let seed = mix64(hash_grid_coord(canonical_coord) ^ 0xc6a4_a793_5bd1_e995);
    let base = metric_vector.first().copied().unwrap_or(0.5);
    (0..64)
        .map(|index| {
            let local = mix64(seed ^ index as u64);
            let jitter = ((local >> 40) as f32) / ((1_u32 << 24) as f32) - 0.5;
            (base + jitter * 0.12).clamp(0.0, 1.0)
        })
        .collect()
}

fn hash_grid_coord(canonical_coord: &GridVec) -> u64 {
    let mut state = 0xc6a4_a793_5bd1_e995_u64;
    for xyz in canonical_coord.to_raw_vec_3d() {
        state = mix64(state ^ fold_signed(xyz.x));
        state = mix64(state ^ fold_signed(xyz.y));
        state = mix64(state ^ fold_signed(xyz.z));
    }
    state
}

#[inline]
fn fold_signed(value: i32) -> u64 {
    value as i64 as u64
}

#[inline]
fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::phenomenon::PhenomenonId;
    use crate::usf::scale::Scale;

    #[test]
    fn transition_policy_thresholds_control_refine_and_coarsen() {
        let policy = SubstrateTransitionPolicy {
            refine_energy_threshold: 0.8,
            refine_instability_threshold: 0.8,
            refine_gradient_threshold: 0.8,
            coarsen_energy_threshold: 0.2,
            coarsen_instability_threshold: 0.2,
            coarsen_gradient_threshold: 0.2,
            delegate_partition_value_threshold: 0.65,
            palette_instability_threshold: 0.33,
            gradient_threshold: 0.24,
        };
        let refine_state = SubstrateRefinementState {
            energy: 0.81,
            instability: 0.1,
            gradient: 0.1,
        };
        let coarsen_state = SubstrateRefinementState {
            energy: 0.1,
            instability: 0.1,
            gradient: 0.1,
        };
        assert!(policy.should_refine(refine_state));
        assert!(!policy.should_coarsen(refine_state));
        assert!(!policy.should_refine(coarsen_state));
        assert!(policy.should_coarsen(coarsen_state));
    }

    #[test]
    fn choose_leaf_representation_honors_partition_delegate_threshold() {
        let coord = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let policy = SubstrateTransitionPolicy {
            delegate_partition_value_threshold: 0.6,
            ..SubstrateTransitionPolicy::default()
        };
        let contributions = vec![ModelProjectionContribution {
            phenomenon_id: PhenomenonId(7),
            model_id: "demo".to_string(),
            topology: PhenomenonModelTopology::PartitionedByChunk,
            partition_key: Some(11),
            metric_index: 0,
            value: 0.61,
        }];
        let leaf = choose_leaf_representation(
            &coord,
            &[0.5],
            &contributions,
            SubstrateRefinementState {
                energy: 0.1,
                instability: 0.1,
                gradient: 0.1,
            },
            policy,
        );
        match leaf {
            SubstrateLeafContainer::DelegatedToPhenomenon {
                phenomenon_id, scale_index, ..
            } => {
                assert_eq!(phenomenon_id, PhenomenonId(7));
                assert_eq!(scale_index, Scale::MAX.index_from_top());
            }
            other => panic!("expected delegated leaf, got {:?}", other),
        }
    }
}
