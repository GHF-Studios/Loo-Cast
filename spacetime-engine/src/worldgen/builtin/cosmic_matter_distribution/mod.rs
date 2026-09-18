//! Built-in `cosmic_matter_distribution` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct CosmicMatterDistributionState {
    pub density_contrast: f32,
    pub filament_strength: f32,
    pub collapse_potential: f32,
    pub void_strength: f32,
}

pub(super) struct CosmicMatterDistributionRule;

impl PhenomenonRule for CosmicMatterDistributionRule {
    fn id(&self) -> PhenomenonId {
        COSMIC_MATTER_DISTRIBUTION
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(20..=24).contains(&scale) {
            return None;
        }

        let state = if scale == 24 {
            let cosmology =
                current_state::<CosmologicalBackgroundState>(current, COSMOLOGICAL_BACKGROUND)?;
            let contrast = cosmology.density_contrast * 80.0;
            CosmicMatterDistributionState {
                density_contrast: contrast,
                filament_strength: (0.30
                    + contrast.abs() * 4.0
                    + signed_noise(context, 0xC05C_0001) * 0.08)
                    .clamp(0.0, 1.0),
                collapse_potential: (0.45
                    + contrast * 3.0
                    + signed_noise(context, 0xC05C_0002) * 0.12)
                    .clamp(0.0, 1.0),
                void_strength: (0.45 - contrast * 2.0 + signed_noise(context, 0xC05C_0003) * 0.12)
                    .clamp(0.0, 1.0),
            }
        } else {
            let parent =
                parent_state::<CosmicMatterDistributionState>(parent, COSMIC_MATTER_DISTRIBUTION)?;
            CosmicMatterDistributionState {
                density_contrast: parent.density_contrast
                    + signed_noise(context, 0xC05C_1001) * 0.08,
                filament_strength: (parent.filament_strength
                    + 0.08
                    + signed_noise(context, 0xC05C_1002) * 0.07)
                    .clamp(0.0, 1.0),
                collapse_potential: (parent.collapse_potential
                    + signed_noise(context, 0xC05C_1003) * 0.11)
                    .clamp(0.0, 1.0),
                void_strength: (parent.void_strength + signed_noise(context, 0xC05C_1004) * 0.09)
                    .clamp(0.0, 1.0),
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "web={:.2}, collapse={:.2}, void={:.2}",
                state.filament_strength, state.collapse_potential, state.void_strength
            ),
        ))
    }
}
