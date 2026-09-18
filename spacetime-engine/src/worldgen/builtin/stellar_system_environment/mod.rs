//! Built-in `stellar_system_environment` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct StellarSystemEnvironmentState {
    pub host_mass_solar: f32,
    pub metallicity: f32,
    pub system_age_gyr: f32,
    pub disk_mass_fraction: f32,
    pub heavy_element_budget: f32,
}

pub(super) struct StellarSystemEnvironmentRule;

impl PhenomenonRule for StellarSystemEnvironmentRule {
    fn id(&self) -> PhenomenonId {
        STELLAR_SYSTEM_ENVIRONMENT
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(8..=14).contains(&scale) {
            return None;
        }

        let state = if scale == 14 {
            let galaxy = current_state::<GalaxyInterstellarMediumState>(
                current,
                GALAXY_INTERSTELLAR_MEDIUM,
            )?;
            StellarSystemEnvironmentState {
                // Deliberately Solar-ish bootstrap bias for the first playable branch.
                host_mass_solar: (0.95 + signed_noise(context, 0x57E1_0001) * 0.18)
                    .clamp(0.55, 1.45),
                metallicity: (galaxy.metallicity * 0.55 + 0.007).clamp(0.004, 0.03),
                system_age_gyr: (4.6 + signed_noise(context, 0x57E1_0002) * 1.3).clamp(1.0, 10.0),
                disk_mass_fraction: (0.025
                    + galaxy.gas_fraction * 0.035
                    + signed_noise(context, 0x57E1_0003) * 0.01)
                    .clamp(0.005, 0.12),
                heavy_element_budget: (galaxy.metallicity / 0.02).clamp(0.1, 2.0),
            }
        } else {
            let parent =
                parent_state::<StellarSystemEnvironmentState>(parent, STELLAR_SYSTEM_ENVIRONMENT)?;
            StellarSystemEnvironmentState {
                host_mass_solar: parent.host_mass_solar,
                metallicity: parent.metallicity,
                system_age_gyr: parent.system_age_gyr,
                disk_mass_fraction: (parent.disk_mass_fraction
                    + signed_noise(context, 0x57E1_1001) * 0.004)
                    .clamp(0.002, 0.15),
                heavy_element_budget: (parent.heavy_element_budget
                    + signed_noise(context, 0x57E1_1002) * 0.06)
                    .clamp(0.05, 2.5),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "host={:.2} solar masses, age={:.2} Gyr, metals={:.2}",
                state.host_mass_solar, state.system_age_gyr, state.heavy_element_budget
            ),
        ))
    }
}
