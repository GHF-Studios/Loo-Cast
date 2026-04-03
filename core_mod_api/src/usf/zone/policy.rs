use crate::usf::scale::Scale;

use super::resources::{ZoneBehaviorRegistry, ZonePhenomenonSelectionStrategy, ZonePhenomenonSupport, ZoneSelectionRuntimeState};
use super::types::ZoneId;

pub fn select_supported_phenomenon_for_zone(
    zone_id: &ZoneId,
    registry: &ZoneBehaviorRegistry,
    parent_selected_phenomenon_id: Option<&str>,
    active_scale: Scale,
    selection_runtime_state: &mut ZoneSelectionRuntimeState,
) -> Option<ZonePhenomenonSupport> {
    let supports = registry.supports_for_zone(&zone_id.zone_type).unwrap_or_else(|| {
        panic!(
            "USF zone realization failed: no supported phenomena for zone '{}'. Add support entries in '*.zone.rhai'.",
            zone_id.zone_type.0
        )
    });
    let selection_policy = registry.selection_policy_for_zone(&zone_id.zone_type).unwrap_or_else(|| {
        panic!(
            "USF zone realization failed: no selection policy for zone '{}'. Define one in '*.zone.rhai'.",
            zone_id.zone_type.0
        )
    });

    let eligible_supports = supports.to_vec();
    if eligible_supports.is_empty() {
        return None;
    }

    // Cross-scale continuity rule: if parent zone realization already selected a
    // phenomenon that is supported here, prefer it to avoid abrupt cross-scale
    // seams at zone boundaries.
    if let Some(parent_phenomenon_id) = parent_selected_phenomenon_id {
        if let Some(inherited) = eligible_supports
            .iter()
            .find(|support| support.phenomenon_id.eq_ignore_ascii_case(parent_phenomenon_id))
        {
            return Some(inherited.clone());
        }
    }

    let highest_priority = eligible_supports
        .iter()
        .map(|support| support.priority)
        .max()
        .expect("eligible_supports contains at least one entry");
    let top_supports = eligible_supports
        .iter()
        .filter(|support| support.priority == highest_priority)
        .cloned()
        .collect::<Vec<_>>();
    if top_supports.len() == 1 {
        return Some(top_supports[0].clone());
    }

    match selection_policy.strategy {
        ZonePhenomenonSelectionStrategy::WeightedTopPriority => {
            let total_weight = top_supports.iter().map(|support| support.weight.max(0.0)).sum::<f32>();
            if total_weight <= f32::EPSILON {
                let seed = mix64(deterministic_phenomenon_seed(zone_id, active_scale) ^ 0x5f37_59df_5a8e_8b4c);
                let index = (seed % top_supports.len() as u64) as usize;
                return Some(top_supports[index].clone());
            }

            let seed = mix64(deterministic_phenomenon_seed(zone_id, active_scale) ^ 0x2d1b_2fa1_d4b4_12cf);
            let normalized = (seed as f64) / (u64::MAX as f64);
            let mut target = (normalized * (total_weight as f64)) as f32;
            for support in &top_supports {
                target -= support.weight.max(0.0);
                if target <= 0.0 {
                    return Some(support.clone());
                }
            }
            Some(top_supports.last().expect("top_supports contains at least one entry").clone())
        }
        ZonePhenomenonSelectionStrategy::HighestWeightTopPriority => {
            let highest_weight = top_supports.iter().map(|support| support.weight).fold(f32::NEG_INFINITY, f32::max);
            let mut candidates = top_supports
                .iter()
                .filter(|support| (support.weight - highest_weight).abs() <= f32::EPSILON)
                .cloned()
                .collect::<Vec<_>>();
            candidates.sort_by(|a, b| a.phenomenon_id.cmp(&b.phenomenon_id));
            Some(candidates.first().expect("candidates contains at least one entry").clone())
        }
        ZonePhenomenonSelectionStrategy::RoundRobinTopPriority => {
            let mut candidates = top_supports;
            candidates.sort_by(|a, b| a.phenomenon_id.cmp(&b.phenomenon_id));
            let cursor_key = format!(
                "{}::{}::{}",
                zone_id.scale.index_from_top(),
                zone_id.zone_type.0.to_ascii_lowercase(),
                active_scale.index_from_top()
            );
            let cursor = selection_runtime_state.round_robin_cursor_by_zone_key.entry(cursor_key).or_insert(0);
            let index = (*cursor % candidates.len() as u64) as usize;
            *cursor = cursor.wrapping_add(1);
            Some(candidates[index].clone())
        }
    }
}

fn deterministic_phenomenon_seed(zone_id: &ZoneId, active_scale: Scale) -> u64 {
    let mut state = mix64(0x9e37_79b9_7f4a_7c15 ^ zone_id.scale.index_from_top() as u64);
    for byte in zone_id.zone_type.0.as_bytes() {
        state = mix64(state ^ *byte as u64);
    }
    state = mix64(state ^ zone_id.stable_region_id.0);
    state = mix64(state ^ active_scale.index_from_top() as u64);

    if state == 0 {
        return 1;
    }
    state
}

#[inline]
fn mix64(mut state: u64) -> u64 {
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::zone::{ZonePhenomenonSpawnPolicy, ZoneSelectionPolicy, ZoneTypeId};
    use std::collections::HashMap;

    fn round_robin_registry() -> ZoneBehaviorRegistry {
        let zone_type = ZoneTypeId::new("mystic");
        let mut registry = ZoneBehaviorRegistry {
            phenomenon_support_by_zone: HashMap::new(),
            selection_policy_by_zone: HashMap::new(),
            density_profile_by_zone: HashMap::new(),
        };
        registry.phenomenon_support_by_zone.insert(
            zone_type.clone(),
            vec![
                ZonePhenomenonSupport {
                    phenomenon_id: "phenomenon.a".to_string(),
                    priority: 100,
                    weight: 1.0,
                    spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                },
                ZonePhenomenonSupport {
                    phenomenon_id: "phenomenon.b".to_string(),
                    priority: 100,
                    weight: 1.0,
                    spawn_policy: ZonePhenomenonSpawnPolicy::SinglePerZone,
                },
            ],
        );
        registry.selection_policy_by_zone.insert(
            zone_type,
            ZoneSelectionPolicy {
                strategy: ZonePhenomenonSelectionStrategy::RoundRobinTopPriority,
            },
        );
        registry
    }

    #[test]
    fn round_robin_selection_rotates_supports() {
        let registry = round_robin_registry();
        let zone_id = ZoneId {
            scale: Scale::MAX,
            zone_type: ZoneTypeId::new("mystic"),
            stable_region_id: crate::usf::zone::StableRegionId(1),
        };
        let mut runtime_state = ZoneSelectionRuntimeState::default();

        let first =
            select_supported_phenomenon_for_zone(&zone_id, &registry, None, Scale::MAX, &mut runtime_state).expect("first selection should resolve");
        let second =
            select_supported_phenomenon_for_zone(&zone_id, &registry, None, Scale::MAX, &mut runtime_state).expect("second selection should resolve");

        assert_ne!(first.phenomenon_id, second.phenomenon_id);
    }

    #[test]
    fn parent_selected_phenomenon_is_preferred_when_supported() {
        let registry = round_robin_registry();
        let zone_id = ZoneId {
            scale: Scale::MAX,
            zone_type: ZoneTypeId::new("mystic"),
            stable_region_id: crate::usf::zone::StableRegionId(2),
        };
        let mut runtime_state = ZoneSelectionRuntimeState::default();

        let selected = select_supported_phenomenon_for_zone(
            &zone_id,
            &registry,
            Some("phenomenon.b"),
            Scale::MAX,
            &mut runtime_state,
        )
        .expect("selection should resolve");

        assert_eq!(selected.phenomenon_id, "phenomenon.b");
    }
}
