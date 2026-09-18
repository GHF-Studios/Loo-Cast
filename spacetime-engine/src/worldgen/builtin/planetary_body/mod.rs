//! Built-in `planetary_body` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct PlanetaryBodyState {
    pub body_mass_earth: f32,
    pub radius_earth: f32,
    pub volatile_fraction: f32,
    pub water_inventory: f32,
    pub internal_heat: f32,
    pub insolation: f32,
}

pub(super) struct PlanetaryBodyRule;

impl PhenomenonRule for PlanetaryBodyRule {
    fn id(&self) -> PhenomenonId {
        PLANETARY_BODY
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(4..=8).contains(&scale) {
            return None;
        }

        let state = if scale == 8 {
            let system = current_state::<StellarSystemEnvironmentState>(
                current,
                STELLAR_SYSTEM_ENVIRONMENT,
            )?;
            PlanetaryBodyState {
                // First branch intentionally biases toward an Earth-like playable body.
                body_mass_earth: (1.0 + signed_noise(context, 0xB0D1_0001) * 0.22).clamp(0.55, 1.8),
                radius_earth: (1.0 + signed_noise(context, 0xB0D1_0002) * 0.11).clamp(0.7, 1.35),
                volatile_fraction: (0.35
                    + system.heavy_element_budget * 0.12
                    + signed_noise(context, 0xB0D1_0003) * 0.10)
                    .clamp(0.05, 0.8),
                water_inventory: (0.55 + signed_noise(context, 0xB0D1_0004) * 0.20)
                    .clamp(0.05, 0.95),
                internal_heat: (0.55 + signed_noise(context, 0xB0D1_0005) * 0.15).clamp(0.1, 1.0),
                insolation: (1.0 + signed_noise(context, 0xB0D1_0006) * 0.10).clamp(0.75, 1.25),
            }
        } else {
            let parent = parent_state::<PlanetaryBodyState>(parent, PLANETARY_BODY)?;
            PlanetaryBodyState {
                body_mass_earth: parent.body_mass_earth,
                radius_earth: parent.radius_earth,
                volatile_fraction: (parent.volatile_fraction
                    + signed_noise(context, 0xB0D1_1001) * 0.025)
                    .clamp(0.02, 0.9),
                water_inventory: (parent.water_inventory
                    + signed_noise(context, 0xB0D1_1002) * 0.035)
                    .clamp(0.0, 1.0),
                internal_heat: (parent.internal_heat + signed_noise(context, 0xB0D1_1003) * 0.025)
                    .clamp(0.0, 1.0),
                insolation: (parent.insolation + signed_noise(context, 0xB0D1_1004) * 0.02)
                    .clamp(0.2, 2.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "mass={:.2} Earth, radius={:.2} Earth, water={:.2}, heat={:.2}",
                state.body_mass_earth,
                state.radius_earth,
                state.water_inventory,
                state.internal_heat
            ),
        ))
    }
}
