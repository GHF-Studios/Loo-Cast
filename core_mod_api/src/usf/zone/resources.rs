use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::USF_ZONE_KIND_BY_TYPE;
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
    pub zone_to_phenomenon: HashMap<ZoneId, Entity>,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct ZoneBehaviorRegistry {
    pub phenomenon_kind_by_zone: HashMap<ZoneTypeId, PhenomenonKind>,
}
impl Default for ZoneBehaviorRegistry {
    fn default() -> Self {
        let mut phenomenon_kind_by_zone = HashMap::new();

        // Baseline mappings; scripts can override these at boot.
        phenomenon_kind_by_zone.insert(ZoneTypeId::new("void"), PhenomenonKind::Mandelbulb);
        phenomenon_kind_by_zone.insert(ZoneTypeId::new("arid"), PhenomenonKind::Mandelbulb);
        phenomenon_kind_by_zone.insert(ZoneTypeId::new("alpine"), PhenomenonKind::Mandelbulb);
        phenomenon_kind_by_zone.insert(ZoneTypeId::new("forest"), PhenomenonKind::SierpinskiSponge);
        phenomenon_kind_by_zone.insert(ZoneTypeId::new("wetland"), PhenomenonKind::SierpinskiSponge);

        for (zone_type, kind) in USF_ZONE_KIND_BY_TYPE().lock().unwrap().iter() {
            phenomenon_kind_by_zone.insert(ZoneTypeId::new(zone_type.clone()), PhenomenonKind::from_config_value(kind));
        }

        Self { phenomenon_kind_by_zone }
    }
}
impl ZoneBehaviorRegistry {
    pub fn phenomenon_kind_for_zone(&self, zone_type: &ZoneTypeId) -> Option<PhenomenonKind> {
        self.phenomenon_kind_by_zone.get(zone_type).copied().or_else(|| {
            let normalized = ZoneTypeId::new(zone_type.0.trim().to_ascii_lowercase());
            self.phenomenon_kind_by_zone.get(&normalized).copied()
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
