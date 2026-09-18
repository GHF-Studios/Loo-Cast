//! Built-in `material_substrate` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct MaterialSubstrateState {
    pub clay_fraction: f32,
    pub water_content: f32,
    pub porosity: f32,
    pub compaction: f32,
    pub temperature_c: f32,
    pub fracture: f32,
}

pub(super) struct MaterialSubstrateRule;

impl PhenomenonRule for MaterialSubstrateRule {
    fn id(&self) -> PhenomenonId {
        MATERIAL_SUBSTRATE
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        _parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=1).contains(&scale) {
            return None;
        }

        let geology =
            current_state::<GeologyClimateHydrologyState>(current, GEOLOGY_CLIMATE_HYDROLOGY)?;
        let state = MaterialSubstrateState {
            clay_fraction: geology.clay_fraction,
            water_content: (geology.moisture * 0.72 + signed_noise(context, 0xAA71_0001) * 0.08)
                .clamp(0.0, 1.0),
            porosity: (0.28
                + geology.clay_fraction * 0.22
                + signed_noise(context, 0xAA71_0002) * 0.06)
                .clamp(0.08, 0.75),
            compaction: (0.62
                + geology.rockiness * 0.18
                + signed_noise(context, 0xAA71_0003) * 0.08)
                .clamp(0.0, 1.0),
            temperature_c: geology.mean_temperature_c,
            fracture: (0.18
                + geology.tectonic_activity * 0.35
                + signed_noise(context, 0xAA71_0004) * 0.10)
                .clamp(0.0, 1.0),
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "clay={:.2}, water={:.2}, porosity={:.2}, compact={:.2}",
                state.clay_fraction, state.water_content, state.porosity, state.compaction
            ),
        ))
    }
}
