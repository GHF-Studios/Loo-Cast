//! Transactional 2:1 leaf refinement and rollback.

use super::super::*;
use super::input::target_resolution_at_distance;

pub(in crate::voxel::resolution::live) fn block_sort_key(
    key: CelestialClipmapBlockKey,
) -> (i16, i32, i32, i32) {
    (
        key.resolution.binary_exponent(),
        key.coord.x,
        key.coord.y,
        key.coord.z,
    )
}

//
// LOD is a genuinely observer-centered 3D shell field. Predictive validity may
// inflate those shells slightly so useful work survives motion/build latency,
// but semantic surface clearance must NOT be subtracted from distance: doing so
// turns sqrt(h^2 + r^2) into sqrt(h^2 + r^2) - h, which makes the terrain
// footprint grow with altitude instead of shrink and eventually disappear.
#[inline]
pub(super) fn effective_observer_lod_distance_metres(
    distance_metres: f64,
    validity_radius_metres: f64,
) -> f64 {
    (distance_metres - validity_radius_metres.max(0.0)).max(0.0)
}

pub(super) fn refinement_candidate(
    key: CelestialClipmapBlockKey,
    finest: VoxelPresentationResolution,
    observer_anchor_local: DVec3,
    validity_radius_metres: f64,
) -> Option<ClipmapRefinementCandidate> {
    if key.resolution <= finest {
        return None;
    }

    let distance = block_distance_to_point(key, observer_anchor_local);
    let effective_distance =
        effective_observer_lod_distance_metres(distance, validity_radius_metres);
    let target = target_resolution_at_distance(finest, effective_distance);
    let refinement_debt = key
        .resolution
        .binary_exponent()
        .saturating_sub(target.binary_exponent());
    let projected_error = key.spacing_metres() / distance.max(key.spacing_metres());

    (refinement_debt > 0).then_some(ClipmapRefinementCandidate {
        inside_validity: distance <= validity_radius_metres.max(0.0),
        distance,
        projected_error,
        refinement_debt,
        key,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LeafRefinementOutcome {
    Refined,
    Retained,
    Unavailable,
}

fn refine_leaf_indexed(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> LeafRefinementOutcome {
    inserted.clear();
    if !leaves.remove(&parent) {
        return LeafRefinementOutcome::Unavailable;
    }

    let Some(children) = parent.children() else {
        leaves.insert(parent);
        return LeafRefinementOutcome::Unavailable;
    };

    for child in children {
        if !input.visibility.demands_block(child) {
            continue;
        }
        if block_contains_local_point(child, input.planning_anchor_local)
            || surface_cache.refinement_intersects(child, classifier)
        {
            leaves.insert(child);
            inserted.push(child);
        }
    }

    if inserted.is_empty() {
        leaves.insert(parent);
        LeafRefinementOutcome::Retained
    } else {
        LeafRefinementOutcome::Refined
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct LeafRefinementMutation {
    parent: CelestialClipmapBlockKey,
    children: [Option<CelestialClipmapBlockKey>; 8],
}

pub(super) fn refine_leaf_transactional(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    parent: CelestialClipmapBlockKey,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    inserted: &mut Vec<CelestialClipmapBlockKey>,
    journal: &mut Vec<LeafRefinementMutation>,
) -> LeafRefinementOutcome {
    let outcome = refine_leaf_indexed(input, leaves, parent, surface_cache, classifier, inserted);

    if outcome == LeafRefinementOutcome::Refined {
        let mut children = [None; 8];
        for (slot, child) in children.iter_mut().zip(inserted.iter().copied()) {
            *slot = Some(child);
        }
        journal.push(LeafRefinementMutation { parent, children });
    }
    outcome
}

pub(super) fn rollback_leaf_refinements(
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    journal: &[LeafRefinementMutation],
) {
    for mutation in journal.iter().rev() {
        for child in mutation.children.into_iter().flatten() {
            leaves.remove(&child);
        }
        leaves.insert(mutation.parent);
    }
}

/// Restore the 2:1 invariant only around leaves changed by this wave.

pub(super) fn balance_leaves_2_to_1_from_seeds(
    input: CelestialClipmapPlanInput,
    leaves: &mut HashSet<CelestialClipmapBlockKey>,
    surface_cache: &mut CelestialClipmapSurfaceCache,
    classifier: &ClipmapBoundaryClassifier<'_>,
    maximum_exponent: i16,
    maximum_leaves: usize,
    seeds: &[CelestialClipmapBlockKey],
    journal: &mut Vec<LeafRefinementMutation>,
    balanced_inserted: &mut Vec<CelestialClipmapBlockKey>,
) -> bool {
    let mut queue = VecDeque::<CelestialClipmapBlockKey>::with_capacity(seeds.len().max(64));
    let mut queued =
        HashSet::<CelestialClipmapBlockKey>::with_capacity(seeds.len().saturating_mul(2).max(64));

    for &seed in seeds {
        if queued.insert(seed) {
            queue.push_back(seed);
        }
    }

    let mut inserted = Vec::<CelestialClipmapBlockKey>::with_capacity(8);
    balanced_inserted.clear();

    while let Some(key) = queue.pop_front() {
        queued.remove(&key);
        if !leaves.contains(&key) {
            continue;
        }

        for (face, _) in CLIPMAP_FACE_DIRECTIONS {
            let Some(neighbor) = same_or_coarser_face_neighbor(leaves, key, face, maximum_exponent)
            else {
                continue;
            };
            let difference = i32::from(neighbor.resolution.binary_exponent())
                - i32::from(key.resolution.binary_exponent());
            if difference <= 1 {
                continue;
            }
            if leaves.len().saturating_add(7) > maximum_leaves {
                return false;
            }

            match refine_leaf_transactional(
                input,
                leaves,
                neighbor,
                surface_cache,
                classifier,
                &mut inserted,
                journal,
            ) {
                LeafRefinementOutcome::Refined => {
                    for &child in &inserted {
                        if queued.insert(child) {
                            queue.push_back(child);
                        }
                    }
                    balanced_inserted.extend(inserted.iter().copied());
                }
                LeafRefinementOutcome::Retained => return false,
                LeafRefinementOutcome::Unavailable => return false,
            }

            if queued.insert(key) {
                queue.push_back(key);
            }
            break;
        }
    }

    true
}
