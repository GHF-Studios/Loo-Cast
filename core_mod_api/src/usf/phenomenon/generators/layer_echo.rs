use crate::bevy::prelude::*;

use crate::usf::phenomenon::generator::{
    BuildStateInput, MeshWindowInput, PhenomenonChildPlan, PhenomenonGenerator, PhenomenonMeshWindow, PhenomenonStateSnapshot, PlanChildrenInput,
};
#[cfg(test)]
use crate::usf::phenomenon::types::{PhenomenonId, PhenomenonLineage, PhenomenonNodeKey, PhenomenonNodeSeed};
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;

#[derive(Debug, Clone, Copy)]
pub struct LayerEchoGenerator {
    pub channel_decay: f32,
    pub echo_gain: f32,
    pub max_branching: u32,
}

const METRIC_PHASE_WRAP: f32 = 64.0;

impl Default for LayerEchoGenerator {
    fn default() -> Self {
        Self {
            channel_decay: 0.72,
            echo_gain: 0.41,
            max_branching: 8,
        }
    }
}

impl PhenomenonGenerator for LayerEchoGenerator {
    fn build_state(&self, input: BuildStateInput<'_>) -> PhenomenonStateSnapshot {
        let seed = input.key.deterministic_seed();
        let root_seed = input.parent_state.map_or(seed, |parent| parent.root_seed);
        let lineage_depth = input.parent_state.map_or(0, |parent| parent.lineage_depth.saturating_add(1));
        let metric_phase = if let Some(parent) = input.parent_state {
            let cell = input.key.local_cell().as_ivec3().as_vec3();
            // Keep a bounded, lineage-derived phase so every node samples one shared metric field.
            wrap_metric_phase(parent.metric_phase * 10.0 + cell * 2.0)
        } else {
            Vec3::ZERO
        };
        let base = seeded_channels(seed.0);

        let channels = if let Some(parent) = input.parent_state {
            // Child state intentionally depends on parent snapshot to encode lineage.
            let cell = input.key.local_cell().as_ivec3().as_vec3();
            let lineage_echo = Vec4::new(cell.x, cell.y, cell.z, input.key.local_index as f32) * 0.013;
            parent.channels * self.channel_decay + base * self.echo_gain + lineage_echo
        } else {
            base
        };

        PhenomenonStateSnapshot {
            seed,
            root_seed,
            lineage_depth,
            metric_phase,
            channels,
        }
    }

    fn sample_density(&self, state: &PhenomenonStateSnapshot, point_local: Vec3) -> f32 {
        let point = point_local.clamp(Vec3::splat(-1.0), Vec3::splat(1.0));
        let metric_point = point + state.metric_phase;

        // One shared metric field across all scales: the root seed defines the global field,
        // while lineage depth selects which detail decades are currently resolved.
        let base_radius = (0.62 + seed_to_unit(state.root_seed.0 ^ 0x4d59_5df4_d0f3_3173) * 0.06).clamp(0.52, 0.74);
        let base_sdf = point.length() - base_radius;

        let depth = state.lineage_depth as i32;
        let max_depth = (Scale::SCALE_LEVEL_COUNT.saturating_sub(1)) as i32;
        let mut detail_sum = 0.0_f32;
        let mut detail_norm = 0.0_f32;

        // Nine contiguous absolute bands centered on the current depth.
        for rel_band in -4..=4 {
            let abs_band = (depth + rel_band).clamp(0, max_depth) as u64;
            let band_seed = splitmix64(state.root_seed.0 ^ abs_band.wrapping_mul(0x9e37_79b9_7f4a_7c15));
            let phase = Vec3::new(
                seed_to_unit(band_seed ^ 0x243f_6a88_85a3_08d3),
                seed_to_unit(band_seed ^ 0x1319_8a2e_0370_7344),
                seed_to_unit(band_seed ^ 0xa409_3822_299f_31d0),
            ) * 0.55;
            let frequency = 10.0_f32.powi(rel_band);
            let band_noise = perlin_noise(band_seed ^ 0x082e_fa98_ec4e_6c89, metric_point * frequency + phase);
            let amplitude = 1.0 / (1.0 + rel_band.abs() as f32);
            detail_sum += band_noise * amplitude;
            detail_norm += amplitude;
        }

        let detail = if detail_norm <= f32::EPSILON { 0.0 } else { detail_sum / detail_norm };
        base_sdf + detail * 0.30
    }

    fn plan_children(&self, input: PlanChildrenInput<'_>) -> Vec<PhenomenonChildPlan> {
        if input.key.scale == Scale::MIN {
            return Vec::new();
        }
        let next_scale = input.key.scale.zoomed_in();

        let offsets = [
            // Keep the primary expansion branch centered to avoid apparent translation drift while scaling.
            LocalCell3::ZERO,
            LocalCell3::new_local(1, 0, 0),
            LocalCell3::new_local(-1, 0, 0),
            LocalCell3::new_local(0, 1, 0),
            LocalCell3::new_local(0, -1, 0),
            LocalCell3::new_local(0, 0, 1),
            LocalCell3::new_local(0, 0, -1),
            LocalCell3::new_local(1, 1, 1),
        ];

        let requested = input.max_children.min(self.max_branching).min(offsets.len() as u32) as usize;
        offsets
            .iter()
            .take(requested)
            .enumerate()
            .map(|(i, offset)| PhenomenonChildPlan {
                local_index: i as u32,
                local_cell: *offset,
                scale: next_scale,
            })
            .collect()
    }

    fn mesh_window(&self, input: MeshWindowInput<'_>) -> PhenomenonMeshWindow {
        let resolution = input.target_resolution.clamp(8, 96);
        PhenomenonMeshWindow {
            lattice_min: IVec3::splat(-16),
            lattice_max: IVec3::splat(16),
            lattice_resolution: UVec3::splat(resolution),
        }
    }
}

fn seeded_channels(seed: u64) -> Vec4 {
    Vec4::new(
        seed_to_unit(seed ^ 0x243f_6a88_85a3_08d3),
        seed_to_unit(seed ^ 0x1319_8a2e_0370_7344),
        seed_to_unit(seed ^ 0xa409_3822_299f_31d0),
        seed_to_unit(seed ^ 0x082e_fa98_ec4e_6c89),
    )
}

fn seed_to_unit(seed: u64) -> f32 {
    let mixed = splitmix64(seed);
    let unit = (mixed as f64) / (u64::MAX as f64);
    (unit as f32) * 2.0 - 1.0
}

fn splitmix64(mut state: u64) -> u64 {
    state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[inline]
fn wrap_metric_phase(value: Vec3) -> Vec3 {
    Vec3::new(
        value.x.rem_euclid(METRIC_PHASE_WRAP),
        value.y.rem_euclid(METRIC_PHASE_WRAP),
        value.z.rem_euclid(METRIC_PHASE_WRAP),
    )
}

#[inline]
fn perlin_fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn perlin_lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
fn perlin_gradient(hash: u64) -> Vec3 {
    match (hash % 12) as u8 {
        0 => Vec3::new(1.0, 1.0, 0.0),
        1 => Vec3::new(-1.0, 1.0, 0.0),
        2 => Vec3::new(1.0, -1.0, 0.0),
        3 => Vec3::new(-1.0, -1.0, 0.0),
        4 => Vec3::new(1.0, 0.0, 1.0),
        5 => Vec3::new(-1.0, 0.0, 1.0),
        6 => Vec3::new(1.0, 0.0, -1.0),
        7 => Vec3::new(-1.0, 0.0, -1.0),
        8 => Vec3::new(0.0, 1.0, 1.0),
        9 => Vec3::new(0.0, -1.0, 1.0),
        10 => Vec3::new(0.0, 1.0, -1.0),
        _ => Vec3::new(0.0, -1.0, -1.0),
    }
}

#[inline]
fn lattice_hash(seed: u64, x: i32, y: i32, z: i32) -> u64 {
    let mut state = seed;
    state ^= (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    state ^= (y as i64 as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= (z as i64 as u64).wrapping_mul(0x94d0_49bb_1331_11eb);
    splitmix64(state)
}

fn perlin_noise(seed: u64, point: Vec3) -> f32 {
    let x0 = point.x.floor() as i32;
    let y0 = point.y.floor() as i32;
    let z0 = point.z.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    let z1 = z0 + 1;

    let tx = point.x - x0 as f32;
    let ty = point.y - y0 as f32;
    let tz = point.z - z0 as f32;

    let g000 = perlin_gradient(lattice_hash(seed, x0, y0, z0));
    let g100 = perlin_gradient(lattice_hash(seed, x1, y0, z0));
    let g010 = perlin_gradient(lattice_hash(seed, x0, y1, z0));
    let g110 = perlin_gradient(lattice_hash(seed, x1, y1, z0));
    let g001 = perlin_gradient(lattice_hash(seed, x0, y0, z1));
    let g101 = perlin_gradient(lattice_hash(seed, x1, y0, z1));
    let g011 = perlin_gradient(lattice_hash(seed, x0, y1, z1));
    let g111 = perlin_gradient(lattice_hash(seed, x1, y1, z1));

    let p000 = Vec3::new(tx, ty, tz);
    let p100 = Vec3::new(tx - 1.0, ty, tz);
    let p010 = Vec3::new(tx, ty - 1.0, tz);
    let p110 = Vec3::new(tx - 1.0, ty - 1.0, tz);
    let p001 = Vec3::new(tx, ty, tz - 1.0);
    let p101 = Vec3::new(tx - 1.0, ty, tz - 1.0);
    let p011 = Vec3::new(tx, ty - 1.0, tz - 1.0);
    let p111 = Vec3::new(tx - 1.0, ty - 1.0, tz - 1.0);

    let n000 = g000.dot(p000);
    let n100 = g100.dot(p100);
    let n010 = g010.dot(p010);
    let n110 = g110.dot(p110);
    let n001 = g001.dot(p001);
    let n101 = g101.dot(p101);
    let n011 = g011.dot(p011);
    let n111 = g111.dot(p111);

    let u = perlin_fade(tx);
    let v = perlin_fade(ty);
    let w = perlin_fade(tz);

    let nx00 = perlin_lerp(n000, n100, u);
    let nx10 = perlin_lerp(n010, n110, u);
    let nx01 = perlin_lerp(n001, n101, u);
    let nx11 = perlin_lerp(n011, n111, u);
    let nxy0 = perlin_lerp(nx00, nx10, v);
    let nxy1 = perlin_lerp(nx01, nx11, v);
    perlin_lerp(nxy0, nxy1, w)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root_key() -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(100),
            scale: Scale::MAX,
            lineage: PhenomenonLineage::root(),
            parent: None,
            local_index: 0,
        }
    }

    fn child_key(parent_key: &PhenomenonNodeKey, child: &PhenomenonChildPlan) -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: parent_key.phenomenon_id,
            scale: child.scale,
            lineage: parent_key.lineage.pushed(child.local_cell),
            parent: Some(parent_key.deterministic_seed()),
            local_index: child.local_index,
        }
    }

    #[test]
    fn layer_echo_child_state_depends_on_parent_snapshot() {
        let generator = LayerEchoGenerator::default();
        let parent_key = root_key();
        let parent = generator.build_state(BuildStateInput {
            key: parent_key.clone(),
            parent_state: None,
        });
        let child_plan = generator.plan_children(PlanChildrenInput {
            key: parent_key.clone(),
            state: &parent,
            max_children: 1,
        })[0];
        let child_key = child_key(&parent_key, &child_plan);

        let with_parent = generator.build_state(BuildStateInput {
            key: child_key.clone(),
            parent_state: Some(&parent),
        });
        let without_parent = generator.build_state(BuildStateInput {
            key: child_key,
            parent_state: None,
        });

        assert_ne!(with_parent.channels, without_parent.channels);
        assert_eq!(with_parent.seed, without_parent.seed);
    }

    #[test]
    fn layer_echo_regenerates_deterministically_for_five_scales() {
        fn regenerate(generator: LayerEchoGenerator) -> Vec<(PhenomenonNodeSeed, Vec4)> {
            let mut out = Vec::new();
            let mut key = root_key();
            let mut state = generator.build_state(BuildStateInput {
                key: key.clone(),
                parent_state: None,
            });
            out.push((state.seed, state.channels));

            for _ in 0..4 {
                let plan = generator.plan_children(PlanChildrenInput {
                    key: key.clone(),
                    state: &state,
                    max_children: 1,
                });
                let Some(first_child) = plan.first().copied() else {
                    break;
                };
                let next_key = child_key(&key, &first_child);
                let next_state = generator.build_state(BuildStateInput {
                    key: next_key.clone(),
                    parent_state: Some(&state),
                });
                out.push((next_state.seed, next_state.channels));
                key = next_key;
                state = next_state;
            }

            out
        }

        let run_a = regenerate(LayerEchoGenerator::default());
        let run_b = regenerate(LayerEchoGenerator::default());

        assert!(run_a.len() >= 5, "expected at least 5 deterministic scales, got {}", run_a.len());
        assert_eq!(run_a, run_b);
    }

    #[test]
    fn layer_echo_regenerates_deterministically_across_full_71_scale_span() {
        fn regenerate_full_span(generator: LayerEchoGenerator) -> Vec<(PhenomenonNodeSeed, Vec4)> {
            let mut out = Vec::new();
            let mut key = root_key();
            let mut state = generator.build_state(BuildStateInput {
                key: key.clone(),
                parent_state: None,
            });
            out.push((state.seed, state.channels));

            for _ in 0..(Scale::SCALE_LEVEL_COUNT as usize - 1) {
                let plan = generator.plan_children(PlanChildrenInput {
                    key: key.clone(),
                    state: &state,
                    max_children: 1,
                });
                let Some(first_child) = plan.first().copied() else {
                    break;
                };
                let next_key = child_key(&key, &first_child);
                let next_state = generator.build_state(BuildStateInput {
                    key: next_key.clone(),
                    parent_state: Some(&state),
                });
                out.push((next_state.seed, next_state.channels));
                key = next_key;
                state = next_state;
            }

            out
        }

        let run_a = regenerate_full_span(LayerEchoGenerator::default());
        let run_b = regenerate_full_span(LayerEchoGenerator::default());
        assert_eq!(run_a.len(), Scale::SCALE_LEVEL_COUNT as usize);
        assert_eq!(run_a, run_b);
    }

    #[test]
    fn layer_echo_parent_child_reproducibility_from_snapshot() {
        let generator = LayerEchoGenerator::default();
        let parent_key = root_key();
        let parent = generator.build_state(BuildStateInput {
            key: parent_key.clone(),
            parent_state: None,
        });
        let child_plan = generator.plan_children(PlanChildrenInput {
            key: parent_key.clone(),
            state: &parent,
            max_children: 1,
        })[0];
        let child_key = child_key(&parent_key, &child_plan);

        let first = generator.build_state(BuildStateInput {
            key: child_key.clone(),
            parent_state: Some(&parent),
        });
        let second = generator.build_state(BuildStateInput {
            key: child_key,
            parent_state: Some(&parent),
        });
        assert_eq!(first, second);
    }

    #[test]
    fn layer_echo_density_contains_stable_surface_shell() {
        let generator = LayerEchoGenerator::default();
        let root = generator.build_state(BuildStateInput {
            key: root_key(),
            parent_state: None,
        });

        let center = generator.sample_density(&root, Vec3::ZERO);
        let corner = generator.sample_density(&root, Vec3::splat(1.0));
        assert!(center < 0.0, "center should stay inside the generated volume, got {center}");
        assert!(corner > 0.0, "far corner should stay outside the generated volume, got {corner}");
    }

    #[test]
    fn layer_echo_primary_child_branch_is_centered_for_scale_stability() {
        let generator = LayerEchoGenerator::default();
        let root_state = generator.build_state(BuildStateInput {
            key: root_key(),
            parent_state: None,
        });
        let plans = generator.plan_children(PlanChildrenInput {
            key: root_key(),
            state: &root_state,
            max_children: 1,
        });
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].local_cell, LocalCell3::ZERO);
    }

    #[test]
    fn layer_echo_child_keeps_root_seed_for_shared_metric_sampling() {
        let generator = LayerEchoGenerator::default();
        let parent_key = root_key();
        let parent = generator.build_state(BuildStateInput {
            key: parent_key.clone(),
            parent_state: None,
        });
        let child_plan = generator.plan_children(PlanChildrenInput {
            key: parent_key.clone(),
            state: &parent,
            max_children: 1,
        })[0];
        let child_key = child_key(&parent_key, &child_plan);
        let child = generator.build_state(BuildStateInput {
            key: child_key,
            parent_state: Some(&parent),
        });

        assert_eq!(child.root_seed, parent.root_seed);
        assert!(child.lineage_depth > parent.lineage_depth);
    }
}
