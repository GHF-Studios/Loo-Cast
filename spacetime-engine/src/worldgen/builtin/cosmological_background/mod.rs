//! Built-in `cosmological_background` worldgen phenomenon.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct CosmologicalBackgroundState {
    pub matter_fraction: f32,
    pub baryon_fraction: f32,
    pub dark_energy_fraction: f32,
    pub background_temperature_k: f32,
    pub density_contrast: f32,
    pub tidal_bias: f32,
}

pub(super) struct CosmologicalBackgroundRule;

impl PhenomenonRule for CosmologicalBackgroundRule {
    fn id(&self) -> PhenomenonId {
        COSMOLOGICAL_BACKGROUND
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        _current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot> {
        let scale = context.spatial_scale().exponent();
        if !(24..=35).contains(&scale) {
            return None;
        }

        let state = if scale == 35 {
            CosmologicalBackgroundState {
                matter_fraction: 0.315,
                baryon_fraction: 0.049,
                dark_energy_fraction: 0.685,
                background_temperature_k: 2.7255,
                density_contrast: signed_noise(context, 0xC05A_0001) * 1.0e-5,
                tidal_bias: signed_noise(context, 0xC05A_0002) * 1.0e-5,
            }
        } else {
            let parent =
                parent_state::<CosmologicalBackgroundState>(parent, COSMOLOGICAL_BACKGROUND)?;
            let depth = (35 - scale) as i32;
            let band_amplitude = 1.0e-5 * 1.55_f32.powi(depth);
            CosmologicalBackgroundState {
                matter_fraction: parent.matter_fraction,
                baryon_fraction: parent.baryon_fraction,
                dark_energy_fraction: parent.dark_energy_fraction,
                background_temperature_k: parent.background_temperature_k,
                density_contrast: parent.density_contrast
                    + signed_noise(context, 0xC05A_1001) * band_amplitude,
                tidal_bias: parent.tidal_bias
                    + signed_noise(context, 0xC05A_1002) * band_amplitude * 0.7,
            }
        };

        Some(PhenomenonSnapshot::new(
            self.id(),
            state,
            format!(
                "δ={:.3e}, tidal={:.3e}, Tcmb={:.4} K",
                state.density_contrast, state.tidal_bias, state.background_temperature_k
            ),
        ))
    }
}
