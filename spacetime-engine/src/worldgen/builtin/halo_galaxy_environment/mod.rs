//! Built-in `halo_galaxy_environment` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct HaloGalaxyEnvironmentState {
    pub halo_mass_bias: f32,
    pub angular_momentum: f32,
    pub baryon_retention: f32,
    pub metallicity: f32,
}

pub(super) struct HaloGalaxyEnvironmentRule;

impl PhenomenonRule for HaloGalaxyEnvironmentRule {
    fn id(&self) -> PhenomenonId {
        HALO_GALAXY_ENVIRONMENT
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(18..=20).contains(&scale) {
            return None;
        }

        let state = if scale == 20 {
            let cosmic = current_state::<CosmicMatterDistributionState>(
                current,
                COSMIC_MATTER_DISTRIBUTION,
            )?;
            HaloGalaxyEnvironmentState {
                halo_mass_bias: (0.35
                    + cosmic.collapse_potential * 0.55
                    + signed_noise(context, 0xA110_0001) * 0.10)
                    .clamp(0.0, 1.0),
                angular_momentum: unit_noise(context, 0xA110_0002),
                baryon_retention: (0.45
                    + cosmic.filament_strength * 0.4
                    + signed_noise(context, 0xA110_0003) * 0.08)
                    .clamp(0.1, 1.0),
                metallicity: (0.006 + cosmic.collapse_potential * 0.010).clamp(0.001, 0.03),
            }
        } else {
            let parent =
                parent_state::<HaloGalaxyEnvironmentState>(parent, HALO_GALAXY_ENVIRONMENT)?;
            HaloGalaxyEnvironmentState {
                halo_mass_bias: (parent.halo_mass_bias + signed_noise(context, 0xA110_1001) * 0.12)
                    .clamp(0.0, 1.0),
                angular_momentum: (parent.angular_momentum * 0.8
                    + unit_noise(context, 0xA110_1002) * 0.2)
                    .clamp(0.0, 1.0),
                baryon_retention: (parent.baryon_retention
                    + signed_noise(context, 0xA110_1003) * 0.06)
                    .clamp(0.1, 1.0),
                metallicity: (parent.metallicity + signed_noise(context, 0xA110_1004) * 0.0015)
                    .clamp(0.001, 0.04),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "halo={:.2}, spin={:.2}, Z={:.3}",
                state.halo_mass_bias, state.angular_momentum, state.metallicity
            ),
        ))
    }
}
