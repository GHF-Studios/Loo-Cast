use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_ZONE_DENSITY_PROFILE_BY_TYPE, USF_ZONE_KIND_BY_TYPE};
use crate::usf::definition::ZoneTypeId;
use crate::usf::phenomenon::PhenomenonKind;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;

use super::types::{ZoneExtent, ZoneId};

#[derive(Resource, Debug, Default)]
pub struct ZoneRuntimeState {
    pub records: HashMap<ZoneId, ZoneExtent>,
    pub entities: HashMap<ZoneId, Entity>,
    pub chunk_to_zone: HashMap<GridVec, ZoneId>,
    pub parent_by_zone: HashMap<ZoneId, ZoneId>,
}

#[derive(Resource, Debug, Default)]
pub struct ZoneRealizationState {
    pub zone_to_phenomenon: HashMap<ZoneId, ZoneRealizedPhenomenon>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRealizedPhenomenon {
    pub phenomenon_entity: Entity,
    pub primary_model_entity: Entity,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub struct ZoneDensityProfile {
    pub density_multiplier: f32,
    pub density_offset: f32,
    pub density_floor: f32,
    pub density_ceil: f32,
}
impl ZoneDensityProfile {
    pub fn normalized_density(&self, normalized_noise: f32) -> f32 {
        let normalized_noise = normalized_noise.clamp(0.0, 1.0);
        (normalized_noise * self.density_multiplier + self.density_offset).clamp(self.density_floor, self.density_ceil)
    }

    pub fn signature(&self) -> u64 {
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        for bits in [
            self.density_multiplier.to_bits() as u64,
            self.density_offset.to_bits() as u64,
            self.density_floor.to_bits() as u64,
            self.density_ceil.to_bits() as u64,
        ] {
            state ^= bits.wrapping_mul(0xbf58_476d_1ce4_e5b9);
            state ^= state >> 30;
            state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
            state ^= state >> 27;
        }
        state
    }
}
impl Default for ZoneDensityProfile {
    fn default() -> Self {
        Self {
            density_multiplier: 1.0,
            density_offset: 0.0,
            density_floor: 0.0,
            density_ceil: 1.0,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct ZoneBehaviorRegistry {
    pub phenomenon_kind_by_zone: HashMap<ZoneTypeId, PhenomenonKind>,
    pub density_profile_by_zone: HashMap<ZoneTypeId, ZoneDensityProfile>,
}
impl Default for ZoneBehaviorRegistry {
    fn default() -> Self {
        let mut phenomenon_kind_by_zone = HashMap::new();
        let mut density_profile_by_zone = HashMap::new();
        let script_kind_entries = USF_ZONE_KIND_BY_TYPE().lock().unwrap().clone();
        let script_density_entries = USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clone();

        // Script-authored entries are authoritative when present.
        if script_kind_entries.is_empty() {
            phenomenon_kind_by_zone.insert(ZoneTypeId::new("void"), PhenomenonKind::Mandelbulb);
            phenomenon_kind_by_zone.insert(ZoneTypeId::new("arid"), PhenomenonKind::Mandelbulb);
            phenomenon_kind_by_zone.insert(ZoneTypeId::new("alpine"), PhenomenonKind::Mandelbulb);
            phenomenon_kind_by_zone.insert(ZoneTypeId::new("forest"), PhenomenonKind::SierpinskiSponge);
            phenomenon_kind_by_zone.insert(ZoneTypeId::new("wetland"), PhenomenonKind::SierpinskiSponge);
        } else {
            for (zone_type, kind) in script_kind_entries {
                phenomenon_kind_by_zone.insert(ZoneTypeId::new(zone_type), PhenomenonKind::from_config_value(&kind));
            }
        }

        if script_density_entries.is_empty() {
            density_profile_by_zone.insert(
                ZoneTypeId::new("void"),
                ZoneDensityProfile {
                    density_multiplier: 0.15,
                    density_offset: 0.0,
                    density_floor: 0.0,
                    density_ceil: 0.25,
                },
            );
            density_profile_by_zone.insert(
                ZoneTypeId::new("arid"),
                ZoneDensityProfile {
                    density_multiplier: 0.45,
                    density_offset: 0.05,
                    density_floor: 0.0,
                    density_ceil: 0.55,
                },
            );
            density_profile_by_zone.insert(
                ZoneTypeId::new("alpine"),
                ZoneDensityProfile {
                    density_multiplier: 0.55,
                    density_offset: 0.10,
                    density_floor: 0.0,
                    density_ceil: 0.72,
                },
            );
            density_profile_by_zone.insert(
                ZoneTypeId::new("forest"),
                ZoneDensityProfile {
                    density_multiplier: 0.72,
                    density_offset: 0.14,
                    density_floor: 0.05,
                    density_ceil: 0.88,
                },
            );
            density_profile_by_zone.insert(
                ZoneTypeId::new("wetland"),
                ZoneDensityProfile {
                    density_multiplier: 0.78,
                    density_offset: 0.20,
                    density_floor: 0.10,
                    density_ceil: 0.95,
                },
            );
        } else {
            for (zone_type, profile) in script_density_entries {
                density_profile_by_zone.insert(
                    ZoneTypeId::new(zone_type),
                    ZoneDensityProfile {
                        density_multiplier: profile.density_multiplier,
                        density_offset: profile.density_offset,
                        density_floor: profile.density_floor,
                        density_ceil: profile.density_ceil,
                    },
                );
            }
        }

        Self {
            phenomenon_kind_by_zone,
            density_profile_by_zone,
        }
    }
}
impl ZoneBehaviorRegistry {
    pub fn phenomenon_kind_for_zone(&self, zone_type: &ZoneTypeId) -> Option<PhenomenonKind> {
        self.phenomenon_kind_by_zone.get(zone_type).copied().or_else(|| {
            let normalized = ZoneTypeId::new(zone_type.0.trim().to_ascii_lowercase());
            self.phenomenon_kind_by_zone.get(&normalized).copied()
        })
    }

    pub fn density_profile_for_zone(&self, zone_type: &ZoneTypeId) -> Option<ZoneDensityProfile> {
        self.density_profile_by_zone.get(zone_type).copied().or_else(|| {
            let normalized = ZoneTypeId::new(zone_type.0.trim().to_ascii_lowercase());
            self.density_profile_by_zone.get(&normalized).copied()
        })
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ZoneTemporalContext {
    pub active_scale: Scale,
}
impl Default for ZoneTemporalContext {
    fn default() -> Self {
        Self { active_scale: Scale::MAX }
    }
}
impl ZoneTemporalContext {
    pub fn time_factor_for_scale(&self, target_scale: Scale) -> f32 {
        time_scale_for_scale(self.active_scale, target_scale)
    }
}

#[inline]
pub fn time_scale_for_levels_above(levels_above: i64) -> f32 {
    if levels_above <= 0 {
        return 1.0;
    }
    let exponent = levels_above.min(30) as i32;
    10.0_f32.powi(-exponent)
}

#[inline]
pub fn time_scale_for_scale_indices(active_scale_index: i64, target_scale_index: i64) -> f32 {
    // Scale::index_from_top: coarser scales have smaller indices.
    // "levels above active" means target is coarser than active.
    let levels_above = (active_scale_index - target_scale_index).max(0);
    time_scale_for_levels_above(levels_above)
}

#[inline]
pub fn time_scale_for_scale(active_scale: Scale, target_scale: Scale) -> f32 {
    time_scale_for_scale_indices(active_scale.index_from_top() as i64, target_scale.index_from_top() as i64)
}
