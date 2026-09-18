//! Built-in `ecology` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct EcologyState {
    pub productivity: f32,
    pub forest_affinity: f32,
    pub grass_affinity: f32,
    pub wetland_affinity: f32,
    pub disturbance: f32,
}

pub(super) struct EcologyRule;

impl PhenomenonRule for EcologyRule {
    fn id(&self) -> PhenomenonId {
        ECOLOGY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=2).contains(&scale) {
            return None;
        }

        let geology =
            current_state::<GeologyClimateHydrologyState>(current, GEOLOGY_CLIMATE_HYDROLOGY)?;
        let temperature_fit =
            (1.0 - ((geology.mean_temperature_c - 16.0) / 28.0).abs()).clamp(0.0, 1.0);
        let base_productivity = geology.moisture * temperature_fit;
        let parent_ecology = parent_state::<EcologyState>(parent, ECOLOGY);
        let inherited = parent_ecology.map_or(base_productivity, |state| state.productivity);
        let productivity = (inherited * 0.55
            + base_productivity * 0.45
            + signed_noise(context, 0xEC01_0001) * 0.08)
            .clamp(0.0, 1.0);
        let disturbance =
            (0.18 + geology.tectonic_activity * 0.12 + signed_noise(context, 0xEC01_0002) * 0.12)
                .clamp(0.0, 1.0);
        let state = EcologyState {
            productivity,
            forest_affinity: (productivity * 0.75 + geology.moisture * 0.25 - disturbance * 0.20)
                .clamp(0.0, 1.0),
            grass_affinity: (0.45 + productivity * 0.30 - geology.moisture * 0.12
                + disturbance * 0.12)
                .clamp(0.0, 1.0),
            wetland_affinity: (geology.moisture * 0.80 + (1.0 - geology.rockiness) * 0.20)
                .clamp(0.0, 1.0),
            disturbance,
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "productivity={:.2}, forest={:.2}, grass={:.2}, wetland={:.2}",
                state.productivity,
                state.forest_affinity,
                state.grass_affinity,
                state.wetland_affinity
            ),
        ))
    }
}
