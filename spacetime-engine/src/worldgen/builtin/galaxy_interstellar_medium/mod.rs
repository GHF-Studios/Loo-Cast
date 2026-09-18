//! Built-in `galaxy_interstellar_medium` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct GalaxyInterstellarMediumState {
    pub stellar_density: f32,
    pub gas_fraction: f32,
    pub metallicity: f32,
    pub turbulence: f32,
    pub star_formation_potential: f32,
}

pub(super) struct GalaxyInterstellarMediumRule;

impl PhenomenonRule for GalaxyInterstellarMediumRule {
    fn id(&self) -> PhenomenonId {
        GALAXY_INTERSTELLAR_MEDIUM
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(14..=18).contains(&scale) {
            return None;
        }

        let state = if scale == 18 {
            let halo =
                current_state::<HaloGalaxyEnvironmentState>(current, HALO_GALAXY_ENVIRONMENT)?;
            GalaxyInterstellarMediumState {
                stellar_density: (0.25 + halo.halo_mass_bias * 0.65).clamp(0.0, 1.0),
                gas_fraction: (0.50
                    + halo.baryon_retention * 0.25
                    + signed_noise(context, 0x6A1A_0001) * 0.10)
                    .clamp(0.05, 0.9),
                metallicity: halo.metallicity,
                turbulence: (0.35
                    + halo.angular_momentum * 0.35
                    + signed_noise(context, 0x6A1A_0002) * 0.12)
                    .clamp(0.0, 1.0),
                star_formation_potential: (0.30
                    + halo.baryon_retention * 0.45
                    + signed_noise(context, 0x6A1A_0003) * 0.15)
                    .clamp(0.0, 1.0),
            }
        } else {
            let parent =
                parent_state::<GalaxyInterstellarMediumState>(parent, GALAXY_INTERSTELLAR_MEDIUM)?;
            GalaxyInterstellarMediumState {
                stellar_density: (parent.stellar_density
                    + signed_noise(context, 0x6A1A_1001) * 0.13)
                    .clamp(0.0, 1.0),
                gas_fraction: (parent.gas_fraction + signed_noise(context, 0x6A1A_1002) * 0.10)
                    .clamp(0.02, 0.95),
                metallicity: (parent.metallicity + signed_noise(context, 0x6A1A_1003) * 0.001)
                    .clamp(0.001, 0.04),
                turbulence: (parent.turbulence + signed_noise(context, 0x6A1A_1004) * 0.12)
                    .clamp(0.0, 1.0),
                star_formation_potential: (parent.star_formation_potential
                    + signed_noise(context, 0x6A1A_1005) * 0.13)
                    .clamp(0.0, 1.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "stars={:.2}, gas={:.2}, SF={:.2}, Z={:.3}",
                state.stellar_density,
                state.gas_fraction,
                state.star_formation_potential,
                state.metallicity
            ),
        ))
    }
}
