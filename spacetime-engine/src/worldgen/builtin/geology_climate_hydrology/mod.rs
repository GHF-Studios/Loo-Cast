//! Built-in `geology_climate_hydrology` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct GeologyClimateHydrologyState {
    pub terrain_seed: u32,
    pub tectonic_activity: f32,
    pub erosion_strength: f32,
    pub mean_temperature_c: f32,
    pub moisture: f32,
    pub rockiness: f32,
    pub cave_potential: f32,
    pub clay_fraction: f32,
    pub local_relief_m: f32,
    pub terrain_frequency: f32,
}

pub(super) struct GeologyClimateHydrologyRule;

impl PhenomenonRule for GeologyClimateHydrologyRule {
    fn id(&self) -> PhenomenonId {
        GEOLOGY_CLIMATE_HYDROLOGY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(0..=4).contains(&scale) {
            return None;
        }

        let state = if scale == 4 {
            let body = current_state::<PlanetaryBodyState>(current, PLANETARY_BODY)?;
            let tectonic =
                (0.35 + body.internal_heat * 0.5 + signed_noise(context, 0x6E01_0001) * 0.10)
                    .clamp(0.0, 1.0);
            let moisture =
                (0.20 + body.water_inventory * 0.65 + signed_noise(context, 0x6E01_0002) * 0.12)
                    .clamp(0.0, 1.0);
            GeologyClimateHydrologyState {
                terrain_seed: context.seed() as u32,
                tectonic_activity: tectonic,
                erosion_strength: (0.25 + moisture * 0.45).clamp(0.0, 1.0),
                mean_temperature_c: 14.0
                    + (body.insolation - 1.0) * 22.0
                    + signed_noise(context, 0x6E01_0003) * 4.0,
                moisture,
                rockiness: (0.35 + tectonic * 0.4 + signed_noise(context, 0x6E01_0004) * 0.12)
                    .clamp(0.0, 1.0),
                cave_potential: (0.25 + moisture * 0.25 + tectonic * 0.15).clamp(0.0, 1.0),
                clay_fraction: (0.20 + moisture * 0.35 + signed_noise(context, 0x6E01_0005) * 0.10)
                    .clamp(0.0, 1.0),
                local_relief_m: 6.0 + tectonic * 6.0,
                terrain_frequency: 0.025,
            }
        } else {
            let parent =
                parent_state::<GeologyClimateHydrologyState>(parent, GEOLOGY_CLIMATE_HYDROLOGY)?;
            GeologyClimateHydrologyState {
                terrain_seed: context.seed() as u32,
                tectonic_activity: (parent.tectonic_activity
                    + signed_noise(context, 0x6E01_1001) * 0.05)
                    .clamp(0.0, 1.0),
                erosion_strength: (parent.erosion_strength
                    + signed_noise(context, 0x6E01_1002) * 0.06)
                    .clamp(0.0, 1.0),
                mean_temperature_c: parent.mean_temperature_c
                    + signed_noise(context, 0x6E01_1003) * 1.5,
                moisture: (parent.moisture + signed_noise(context, 0x6E01_1004) * 0.08)
                    .clamp(0.0, 1.0),
                rockiness: (parent.rockiness + signed_noise(context, 0x6E01_1005) * 0.08)
                    .clamp(0.0, 1.0),
                cave_potential: (parent.cave_potential + signed_noise(context, 0x6E01_1006) * 0.08)
                    .clamp(0.0, 1.0),
                clay_fraction: (parent.clay_fraction + signed_noise(context, 0x6E01_1007) * 0.08)
                    .clamp(0.0, 1.0),
                local_relief_m: (parent.local_relief_m * 0.88
                    + 2.0
                    + signed_noise(context, 0x6E01_1008) * 1.5)
                    .clamp(2.0, 18.0),
                terrain_frequency: (parent.terrain_frequency * 1.17).clamp(0.018, 0.065),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "T={:.1} C, water={:.2}, relief={:.1}m, caves={:.2}, clay={:.2}",
                state.mean_temperature_c,
                state.moisture,
                state.local_relief_m,
                state.cave_potential,
                state.clay_fraction
            ),
        ))
    }
}
