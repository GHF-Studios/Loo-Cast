//! Cross-resolution presentation frontier derived from realized voxel coverage.
//!
//! Same-resolution Surface Nets chunks already stitch through shared canonical
//! samples. This resource owns a different fact: which *ready fine*
//! materialization faces currently border still-coarse presentation context.
//! Future transition geometry consumes this frontier; neither the fine nor the
//! coarse base mesh owns the cross-resolution seam.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::spatial::{
    SpatialScale, UsfCapabilityRealization, UsfCapabilitySet,
    UsfScaleCoverageSnapshot, UsfScaleRoleMask,
};

use super::VoxelMaterializationRuntime;
use super::super::VoxelMaterializationKey;

const NEG_X: u8 = 1 << 0;
const POS_X: u8 = 1 << 1;
const NEG_Y: u8 = 1 << 2;
const POS_Y: u8 = 1 << 3;
const NEG_Z: u8 = 1 << 4;
const POS_Z: u8 = 1 << 5;

const NEIGHBORS: [(IVec3, u8); 6] = [
    (IVec3::new(-1, 0, 0), NEG_X),
    (IVec3::new(1, 0, 0), POS_X),
    (IVec3::new(0, -1, 0), NEG_Y),
    (IVec3::new(0, 1, 0), POS_Y),
    (IVec3::new(0, 0, -1), NEG_Z),
    (IVec3::new(0, 0, 1), POS_Z),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FrontierKey {
    world: Entity,
    authority: Entity,
    fine_scale: SpatialScale,
    key: VoxelMaterializationKey,
}

/// Cardinal faces of one ready fine materialization that border non-fine
/// presentation context.
///
/// The mask deliberately carries no terrain/surface assumption. An empty fine
/// materialization can own frontier faces too: "known empty" is realized truth
/// and may need a transition around an edited cave or excavation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelRefinementFaceMask(u8);

impl VoxelRefinementFaceMask {
    pub(super) const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub(super) const fn bits(self) -> u8 {
        self.0
    }
}

/// One exact fine-cell *coverage* frontier fact.
///
/// This snapshot says where ready fine Scale-Slice presentation borders
/// still-coarse context. It deliberately does **not** define mesh-resolution
/// adjacency. USF Scale is a numerical chart; using S(n)->S(n+1) directly as a
/// 10:1 LOD transition is the architecture this frontier exposed as invalid.
///
/// `voxel::resolution` owns the independent physical sampling ladder. A
/// presentation adapter may project this coverage into that domain, balance it
/// to 2:1 neighbors and only then ask a transition-cell mesher for geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VoxelRefinementFrontierCell {
    world: Entity,
    authority: Entity,
    fine_scale: SpatialScale,
    key: VoxelMaterializationKey,
    exposed_faces: VoxelRefinementFaceMask,
}

impl VoxelRefinementFrontierCell {
    pub(super) const fn world(self) -> Entity {
        self.world
    }

    pub(super) const fn authority(self) -> Entity {
        self.authority
    }

    pub(super) const fn fine_scale(self) -> SpatialScale {
        self.fine_scale
    }

    pub(super) fn coarse_scale(self) -> Option<SpatialScale> {
        SpatialScale::new(self.fine_scale.exponent().checked_add(1)?)
    }

    pub(super) const fn key(self) -> VoxelMaterializationKey {
        self.key
    }

    pub(super) const fn exposed_faces(self) -> VoxelRefinementFaceMask {
        self.exposed_faces
    }
}

/// Sparse, semantically-invalidated refinement frontier.
///
/// `source_coverage_revision` intentionally follows
/// [`UsfScaleCoverageSnapshot::revision`] rather than Bevy change ticks. Camera
/// or system writes cannot make this topology rebuild every frame.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct VoxelRefinementFrontierSet;

#[derive(Resource, Debug, Default)]
pub(super) struct VoxelRefinementFrontierSnapshot {
    source_coverage_revision: u64,
    cells: HashMap<FrontierKey, VoxelRefinementFaceMask>,
}

impl VoxelRefinementFrontierSnapshot {
    pub(super) fn iter(
        &self,
    ) -> impl Iterator<Item = VoxelRefinementFrontierCell> + '_ {
        self.cells.iter().map(|(key, &exposed_faces)| {
            VoxelRefinementFrontierCell {
                world: key.world,
                authority: key.authority,
                fine_scale: key.fine_scale,
                key: key.key,
                exposed_faces,
            }
        })
    }

    pub(super) const fn source_coverage_revision(&self) -> u64 {
        self.source_coverage_revision
    }

    /// Exact exposed-face mask for one ready fine materialization.
    ///
    /// Missing entries are interior/non-frontier cells and therefore retain no
    /// coarse support band.
    pub(super) fn exposed_faces(
        &self,
        world: Entity,
        authority: Entity,
        fine_scale: SpatialScale,
        key: VoxelMaterializationKey,
    ) -> VoxelRefinementFaceMask {
        self.cells
            .get(&FrontierKey {
                world,
                authority,
                fine_scale,
                key,
            })
            .copied()
            .unwrap_or_default()
    }
}

fn sync_refinement_frontier(
    coverage: Res<UsfScaleCoverageSnapshot>,
    runtimes: Query<(
        &VoxelMaterializationRuntime,
        &UsfCapabilityRealization,
    )>,
    mut frontier: ResMut<VoxelRefinementFrontierSnapshot>,
) {
    let coverage_revision = coverage.revision();
    if frontier.source_coverage_revision == coverage_revision {
        return;
    }

    let mut ready = HashSet::<FrontierKey>::new();
    for (runtime, realization) in &runtimes {
        if !realization
            .roles()
            .contains(UsfScaleRoleMask::PRESENTATION)
        {
            continue;
        }

        let fine_scale = realization.scale();
        if fine_scale
            .exponent()
            .checked_add(1)
            .and_then(SpatialScale::new)
            .is_none()
        {
            continue;
        }

        ready.insert(FrontierKey {
            world: runtime.world(),
            authority: realization.authority(),
            fine_scale,
            key: runtime.key(),
        });
    }

    let mut next = HashMap::<FrontierKey, VoxelRefinementFaceMask>::new();

    for key in ready.iter().copied() {
        let mut mask = 0_u8;

        for (delta, bit) in NEIGHBORS {
            let neighbor_ready = key
                .key
                .translated_chunks(delta)
                .ok()
                .is_some_and(|neighbor| {
                    ready.contains(&FrontierKey {
                        world: key.world,
                        authority: key.authority,
                        fine_scale: key.fine_scale,
                        key: neighbor,
                    })
                });

            if !neighbor_ready {
                mask |= bit;
            }
        }

        if mask != 0 {
            next.insert(key, VoxelRefinementFaceMask(mask));
        }
    }

    frontier.cells = next;
    frontier.source_coverage_revision = coverage_revision;
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<VoxelRefinementFrontierSnapshot>()
        .add_systems(
            PostUpdate,
            sync_refinement_frontier
                .in_set(VoxelRefinementFrontierSet)
                .after(UsfCapabilitySet::ReconcileCoverage),
        );
}
