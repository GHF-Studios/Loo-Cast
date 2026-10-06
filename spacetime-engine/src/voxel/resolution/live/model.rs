//! Reconstructible clipmap plans, tickets, coverage and telemetry state.

use super::*;

#[derive(Resource, Default)]
pub(super) struct CelestialClipmapBandDebugMaterials {
    pub(super) by_base_and_relative_level:
        HashMap<(Handle<StandardMaterial>, i16), Handle<VoxelRenderMaterial>>,
}

//
// Keep the binary clipmap systems below Bevy's plain function-system parameter
// arity limit without hiding ownership behind globals. These five resources are
// one cohesive presentation-material dependency and can therefore travel as one
// ordinary ECS SystemParam.
#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct CelestialClipmapMaterialParams<'w> {
    pub(super) standard_materials: Res<'w, Assets<StandardMaterial>>,
    pub(super) render_materials: ResMut<'w, Assets<VoxelRenderMaterial>>,
    pub(super) shader_buffers: ResMut<'w, Assets<ShaderBuffer>>,
    pub(super) library: Res<'w, ProceduralPresentationAssets>,
    pub(super) band_materials: ResMut<'w, CelestialClipmapBandDebugMaterials>,
}

// Sixteen adjacent binary LODs traverse one complete hue revolution. The
// palette deliberately advances slowly enough that neighboring resolution
// shells remain easy to distinguish while broad LOD structure reads as one
// continuous sweep rather than a seven-color repeating traffic light.

impl CelestialClipmapBandDebugMaterials {
    pub(super) fn material_for(
        &mut self,
        standard_materials: &Assets<StandardMaterial>,
        render_materials: &mut Assets<VoxelRenderMaterial>,
        shader_buffers: &mut Assets<ShaderBuffer>,
        debug_grid: &Handle<StandardMaterial>,
        base: &Handle<StandardMaterial>,
        relative_level: i16,
    ) -> Option<Handle<VoxelRenderMaterial>> {
        let diagnostic_band = relative_level.rem_euclid(DEBUG_BAND_COUNT);
        let key = (base.clone(), diagnostic_band);
        if let Some(existing) = self.by_base_and_relative_level.get(&key) {
            return Some(existing.clone());
        }

        let mut tinted = standard_materials.get(base)?.clone();
        // The StandardMaterial tint remains the LOD diagnostic multiplier.
        // If this is the debug-grid marker, the shared voxel shader supplies
        // textureless analytical grid detail in physical metres.
        tinted.base_color = clipmap_band_debug_color(relative_level);
        let grid_uv_metres_per_unit =
            (base == debug_grid).then_some(DEBUG_GRID_BASE_UV_METRES_PER_UNIT);
        let handle = create_voxel_render_material(
            tinted,
            grid_uv_metres_per_unit,
            shader_buffers,
            render_materials,
        );
        self.by_base_and_relative_level.insert(key, handle.clone());
        Some(handle)
    }
}

fn clipmap_band_debug_color(relative_level: i16) -> Color {
    let [r, g, b] = debug_band_rgb(relative_level);
    Color::srgb(r, g, b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct CelestialClipmapBlockSpec {
    pub(super) key: CelestialClipmapBlockKey,
    pub(super) transition_faces: VoxelTransitionFaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct CelestialClipmapPlanKey {
    // Observer coordinates are deliberately absent. They are refinement state,
    // not identity of the body-local presentation hierarchy.
    pub(super) finest_exponent: i16,
    pub(super) coarsest_exponent: i16,
}

#[derive(Debug)]
pub(super) struct CelestialClipmapPlan {
    pub(super) key: CelestialClipmapPlanKey,
    pub(super) field: CelestialVoxelField,
    /// Predicted actual observer position in body-local SI metres.
    ///
    /// This is the 3D LOD/error/motion anchor. Never project it onto terrain:
    /// altitude and cave/interior motion are real components of observer-space
    /// presentation distance.
    pub(super) observer_anchor_local: DVec3,
    /// Nearest canonical semantic boundary point to the observer.
    ///
    /// This owns the mandatory surface-refinement branch only. It does not own
    /// observer-distance LOD selection.
    pub(super) planning_anchor_local: DVec3,
    pub(super) validity_radius_metres: f64,
    pub(super) visibility: ClipmapVisibilityDemand,
    pub(super) generation: u64,
    pub(super) stages: Vec<Vec<CelestialClipmapBlockSpec>>,
    pub(super) stage_index: usize,
    pub(super) desired: Vec<CelestialClipmapBlockSpec>,
    pub(super) desired_set: HashSet<CelestialClipmapBlockSpec>,
    pub(super) completed: HashSet<CelestialClipmapBlockSpec>,
    pub(super) meshful: HashSet<CelestialClipmapBlockSpec>,
    pub(super) committed_specs: HashSet<CelestialClipmapBlockSpec>,
    pub(super) committed_generation: Option<u64>,
}

pub(super) fn seed_clipmap_stage_completion(
    authority: Entity,
    plan: &mut CelestialClipmapPlan,
    active_entities: &HashMap<(Entity, CelestialClipmapBlockSpec), Entity>,
) {
    plan.completed.clear();
    plan.meshful.clear();

    for &spec in &plan.desired {
        let key = (authority, spec);
        if active_entities.contains_key(&key) {
            plan.completed.insert(spec);
            plan.meshful.insert(spec);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ClipmapRefinementCandidate {
    pub(super) inside_validity: bool,
    pub(super) distance: f64,
    pub(super) projected_error: f64,
    pub(super) refinement_debt: i16,
    pub(super) key: CelestialClipmapBlockKey,
}

impl PartialEq for ClipmapRefinementCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.inside_validity == other.inside_validity
            && self.refinement_debt == other.refinement_debt
            && self.projected_error.total_cmp(&other.projected_error) == std::cmp::Ordering::Equal
            && self.distance.total_cmp(&other.distance) == std::cmp::Ordering::Equal
            && self.key == other.key
    }
}

impl Eq for ClipmapRefinementCandidate {}

impl PartialOrd for ClipmapRefinementCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClipmapRefinementCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inside_validity
            .cmp(&other.inside_validity)
            .then_with(|| self.refinement_debt.cmp(&other.refinement_debt))
            .then_with(|| self.projected_error.total_cmp(&other.projected_error))
            .then_with(|| other.distance.total_cmp(&self.distance))
            .then_with(|| self.key.resolution.cmp(&other.key.resolution))
            .then_with(|| block_sort_key(other.key).cmp(&block_sort_key(self.key)))
    }
}

#[derive(Resource, Default)]
pub(super) struct CelestialClipmapRealizations {
    pub(super) next_generation: u64,
    pub(super) gpu_admission_cursor: Option<Entity>,
    pub(super) plans: HashMap<Entity, CelestialClipmapPlan>,
    pub(super) planner_caches: HashMap<Entity, CelestialClipmapSurfaceCache>,
    // Stable presentation shells + async tickets live in this reconstructible
    // registry. They are runtime bookkeeping, not semantic ECS entities.
    pub(super) active_entities: HashMap<(Entity, CelestialClipmapBlockSpec), Entity>,
    pub(super) plan_tasks: Vec<CelestialClipmapPlanBuildTask>,
    pub(super) build_tasks: Vec<CelestialClipmapBuildTask>,

    pub(super) projection_epoch: u64,
    pub(super) frontier_epoch: u64,
    pub(super) projection_pending: Vec<Entity>,
}

impl CelestialClipmapRealizations {
    pub(super) fn next_generation(&mut self) -> u64 {
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        self.next_generation
    }

    pub(super) fn mark_projection_pending(&mut self, entity: Entity) {
        self.projection_pending.push(entity);
        self.projection_epoch = self.projection_epoch.wrapping_add(1).max(1);
    }

    pub(super) fn mark_frontier_changed(&mut self) {
        self.frontier_epoch = self.frontier_epoch.wrapping_add(1).max(1);
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(in crate::voxel) struct CelestialClipmapTelemetry {
    pub(super) plan_requests_total: u64,
    pub(super) cold_plans_total: u64,
    pub(super) warm_replans_total: u64,
    pub(super) visible_blocks: usize,
    pub(super) binary_primary_authorities: usize,
    pub(super) visible_binary_levels: usize,
    pub(super) finest_visible_spacing_metres: Option<f64>,
    pub(super) coarsest_visible_spacing_metres: Option<f64>,
    pub(super) boundary_clearance_metres: Option<f64>,
    pub(super) requested_finest_spacing_metres: Option<f64>,
    pub(super) planned_finest_spacing_metres: Option<f64>,
    pub(super) planner_leaf_budget: usize,
    pub(super) planner_final_leaves: usize,
    pub(super) planner_budget_saturated: bool,
    pub(super) dense_fallback_held: usize,
    pub(super) dense_fallback_retire_ready: usize,
    pub(super) dense_fallback_forced_retire: usize,
    pub(super) committed_focus_lag_metres: Option<f64>,
    pub(super) fresh_plan_accepts_total: u64,
    pub(super) rolling_plan_accepts_total: u64,
    pub(super) stale_plan_drops_total: u64,
}

impl CelestialClipmapTelemetry {
    pub(super) fn record_plan_request(&mut self, warm: bool) {
        self.plan_requests_total = self.plan_requests_total.wrapping_add(1);
        if warm {
            self.warm_replans_total = self.warm_replans_total.wrapping_add(1);
        } else {
            self.cold_plans_total = self.cold_plans_total.wrapping_add(1);
        }
    }

    pub(super) fn record_visible_frontier(
        &mut self,
        visible_blocks: usize,
        binary_primary_authorities: usize,
        visible_levels: &HashSet<i16>,
        finest_spacing: Option<f64>,
        coarsest_spacing: Option<f64>,
    ) {
        self.visible_blocks = visible_blocks;
        self.binary_primary_authorities = binary_primary_authorities;
        self.visible_binary_levels = visible_levels.len();
        self.finest_visible_spacing_metres = finest_spacing;
        self.coarsest_visible_spacing_metres = coarsest_spacing;
    }

    pub(super) fn record_plan_quality(
        &mut self,
        input: CelestialClipmapPlanInput,
        stages: &[Vec<CelestialClipmapBlockSpec>],
    ) {
        self.boundary_clearance_metres = Some(input.clearance_metres);
        self.requested_finest_spacing_metres = Some(input.finest.sample_spacing_metres());
        self.planned_finest_spacing_metres = stages.last().and_then(|stage| {
            stage
                .iter()
                .map(|spec| spec.key.spacing_metres())
                .min_by(f64::total_cmp)
        });
        self.planner_leaf_budget = sparse_frontier_leaf_budget(input);
        self.planner_final_leaves = stages.last().map_or(0, Vec::len);
        self.planner_budget_saturated = self.planner_final_leaves
            >= self.planner_leaf_budget.saturating_sub(7)
            && self
                .planned_finest_spacing_metres
                .zip(self.requested_finest_spacing_metres)
                .is_some_and(|(planned, requested)| planned > requested * 1.001);
    }

    pub(super) fn record_dense_fallbacks(
        &mut self,
        held: usize,
        retire_ready: usize,
        forced_retire: usize,
    ) {
        self.dense_fallback_held = held;
        self.dense_fallback_retire_ready = retire_ready;
        self.dense_fallback_forced_retire = forced_retire;
    }

    pub(super) fn record_plan_task_relevance(&mut self, relevance: ClipmapPlanTaskRelevance) {
        match relevance {
            ClipmapPlanTaskRelevance::Fresh => {
                self.fresh_plan_accepts_total = self.fresh_plan_accepts_total.saturating_add(1);
            }
            ClipmapPlanTaskRelevance::RollingProgress => {
                self.rolling_plan_accepts_total = self.rolling_plan_accepts_total.saturating_add(1);
            }
            ClipmapPlanTaskRelevance::Stale => {
                self.stale_plan_drops_total = self.stale_plan_drops_total.saturating_add(1);
            }
        }
    }

    pub(super) fn record_committed_focus_lag(&mut self, lag_metres: Option<f64>) {
        self.committed_focus_lag_metres = lag_metres;
    }

    pub(in crate::voxel) fn summary(&self) -> String {
        format!(
            "plans={} cold={} warm={} visible={} binary_primary={} levels={} visible={}..{}m clearance={}m requested_finest={}m planned_finest={}m leaf_budget={} leaves={} saturated={} fallback_hold={} fallback_retire={} fallback_forced={} focus_lag={}m plan_fresh={} plan_rolling={} plan_drop={}",
            self.plan_requests_total,
            self.cold_plans_total,
            self.warm_replans_total,
            self.visible_blocks,
            self.binary_primary_authorities,
            self.visible_binary_levels,
            self.finest_visible_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.coarsest_visible_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.boundary_clearance_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.requested_finest_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.planned_finest_spacing_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.planner_leaf_budget,
            self.planner_final_leaves,
            self.planner_budget_saturated,
            self.dense_fallback_held,
            self.dense_fallback_retire_ready,
            self.dense_fallback_forced_retire,
            self.committed_focus_lag_metres
                .map_or_else(|| "-".to_string(), |v| format!("{v:.1}")),
            self.fresh_plan_accepts_total,
            self.rolling_plan_accepts_total,
            self.stale_plan_drops_total,
        )
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct CelestialClipmapBlock {
    pub(super) authority: Entity,
    pub(super) spec: CelestialClipmapBlockSpec,
    pub(super) active: bool,
    pub(super) committed: bool,
    pub(super) projection_ready: bool,
    pub(super) material_relative_level: i16,
}

pub(super) struct CelestialClipmapBuildTask {
    pub(super) authority: Entity,
    pub(super) generation: u64,
    pub(super) spec: CelestialClipmapBlockSpec,
    pub(super) field: CelestialVoxelField,
    pub(super) entity: Entity,
    pub(super) build_id: u64,
}

#[derive(Clone, Copy)]
pub(super) struct PendingGpuAdmission {
    pub(super) authority: Entity,
    pub(super) generation: u64,
    pub(super) spec: CelestialClipmapBlockSpec,
    pub(super) field: CelestialVoxelField,
    pub(super) relative_level: i16,
    pub(super) descriptor: super::super::gpu::GpuTerrainDescriptor,
}

pub(super) struct CelestialClipmapPlanBuildTask {
    pub(super) authority: Entity,
    pub(super) input: CelestialClipmapPlanInput,
    pub(super) field: CelestialVoxelField,
    pub(super) task: VoxelWorkTicket<CelestialClipmapPlanBuildOutput>,
}

pub(super) struct CelestialClipmapPlanBuildOutput {
    pub(super) stages: Option<Vec<Vec<CelestialClipmapBlockSpec>>>,
    pub(super) surface_cache: CelestialClipmapSurfaceCache,
}

/// Presentation-only local coverage committed by the clipmap.
///
/// This is intentionally *not* [`crate::spatial::UsfCapabilityRealization`].
/// Regional presentation may use it to cull counterfeit coarse surface, but it
/// grants no collision/editing/semantic authority.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::voxel) struct CelestialClipmapCoverageCell {
    pub(super) center_local_metres: DVec3,
    pub(super) half_extent_metres: DVec3,
    pub(super) sample_spacing_metres: f64,
}

impl CelestialClipmapCoverageCell {
    pub(in crate::voxel) const fn center_local_metres(self) -> DVec3 {
        self.center_local_metres
    }

    pub(in crate::voxel) const fn sample_spacing_metres(self) -> f64 {
        self.sample_spacing_metres
    }

    pub(super) fn contains_local_point(self, point: DVec3) -> bool {
        let relative = (point - self.center_local_metres).abs();
        relative.x <= self.half_extent_metres.x
            && relative.y <= self.half_extent_metres.y
            && relative.z <= self.half_extent_metres.z
    }

    pub(in crate::voxel) fn inner_radius_metres(self) -> f64 {
        self.half_extent_metres.min_element().max(0.0)
    }

    pub(in crate::voxel) fn outer_radius_metres(self) -> f64 {
        self.half_extent_metres.length()
    }
}

/// Frame-local celestial presentation ownership.
///
/// Dense Surface-Nets terrain is a physical/editable Scale-local working
/// representation. It is allowed to bootstrap presentation only while no
/// coherent binary frontier is available; it is never a geometric LOD child.
///
/// The first committed binary frontier whose complete mesh set projects in
/// the current view takes presentation ownership for the entire authority.
/// Every later quality transition is binary -> binary through the clipmap's
/// own make-before-break frontier transaction. Decimal USF Scale meshes are
/// therefore never stitched, clipped, or locally arbitrated against binary
/// presentation LOD. If binary projection becomes incomplete, dense may
/// reappear only as a whole-authority emergency fallback for that frame.
///
/// This resource is presentation-only and never grants semantic/collision/edit
/// authority.
#[derive(Resource, Debug, Default)]
pub(super) struct CelestialTerrainPresentationState {
    pub(super) binary_primary: HashSet<Entity>,
}

impl CelestialTerrainPresentationState {
    pub(super) fn is_binary_primary(&self, authority: Entity) -> bool {
        self.binary_primary.contains(&authority)
    }

    pub(super) fn replace_binary_primary_from(&mut self, next: &HashSet<Entity>) {
        if self.binary_primary != *next {
            self.binary_primary.clone_from(next);
        }
    }

    pub(super) fn clear(&mut self) {
        self.binary_primary.clear();
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct CelestialClipmapCoverageSnapshot {
    pub(super) revision: u64,
    pub(super) by_authority: HashMap<Entity, Vec<CelestialClipmapCoverageCell>>,
}

impl CelestialClipmapCoverageSnapshot {
    pub(in crate::voxel) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(in crate::voxel) fn for_authority(
        &self,
        authority: Entity,
    ) -> &[CelestialClipmapCoverageCell] {
        self.by_authority
            .get(&authority)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Finest actually-visible binary spacing covering one body-local point.
    ///
    /// This is presentation-query metadata only. It never arbitrates a
    /// decimal Scale-local renderer against the binary LOD hierarchy.
    pub(in crate::voxel) fn finest_spacing_covering(
        &self,
        authority: Entity,
        point_local_metres: DVec3,
    ) -> Option<f64> {
        self.for_authority(authority)
            .iter()
            .copied()
            .filter(|cell| cell.contains_local_point(point_local_metres))
            .map(CelestialClipmapCoverageCell::sample_spacing_metres)
            .min_by(f64::total_cmp)
    }

    pub(super) fn replace_authority(
        &mut self,
        authority: Entity,
        mut coverage: Vec<CelestialClipmapCoverageCell>,
    ) {
        coverage.sort_by(|a, b| {
            a.center_local_metres
                .x
                .total_cmp(&b.center_local_metres.x)
                .then_with(|| a.center_local_metres.y.total_cmp(&b.center_local_metres.y))
                .then_with(|| a.center_local_metres.z.total_cmp(&b.center_local_metres.z))
        });

        if self.by_authority.get(&authority) == Some(&coverage) {
            return;
        }

        self.by_authority.insert(authority, coverage);
        self.revision = self.revision.wrapping_add(1).max(1);
    }

    pub(super) fn replace_authority_from_slice(
        &mut self,
        authority: Entity,
        coverage: &[CelestialClipmapCoverageCell],
    ) {
        if self.by_authority.get(&authority).map(Vec::as_slice) == Some(coverage) {
            return;
        }
        let target = self.by_authority.entry(authority).or_default();
        target.clear();
        target.extend_from_slice(coverage);
        self.revision = self.revision.wrapping_add(1).max(1);
    }

    pub(super) fn remove_authority(&mut self, authority: Entity) {
        if self.by_authority.remove(&authority).is_some() {
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }

    pub(super) fn retain_authorities(&mut self, live: &HashSet<Entity>) {
        let before = self.by_authority.len();
        self.by_authority
            .retain(|authority, _| live.contains(authority));
        if self.by_authority.len() != before {
            self.revision = self.revision.wrapping_add(1).max(1);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CelestialClipmapPlanInput {
    pub(super) key: CelestialClipmapPlanKey,
    pub(super) observer_anchor_local: DVec3,
    pub(super) planning_anchor_local: DVec3,
    pub(super) validity_radius_metres: f64,
    pub(super) visibility: ClipmapVisibilityDemand,
    pub(super) clearance_metres: f64,
    pub(super) finest: VoxelPresentationResolution,
    pub(super) coarsest: VoxelPresentationResolution,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ClipmapProjectedAuthorityFrame {
    pub(super) relative_metres: DVec3,
    pub(super) orientation: bevy::math::DQuat,
    pub(super) rotation: Quat,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ClipmapAuthorityProjectionStamp {
    pub(super) origin: UsfPosition,
    pub(super) orientation: bevy::math::DQuat,
    pub(super) presentation_material: Handle<StandardMaterial>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ClipmapViewProjectionStamp {
    pub(super) anchor: UsfPosition,
    pub(super) metre_to_view_f64: f64,
    pub(super) projection_eye: DVec3,
    pub(super) presentation_origin: Vec3,
}

#[derive(Default)]
pub(super) struct CelestialClipmapTransformScratch {
    pub(super) frames: HashMap<Entity, ClipmapProjectedAuthorityFrame>,
    pub(super) counts: HashMap<Entity, (usize, usize)>,
    pub(super) binary_primary: HashSet<Entity>,
    pub(super) projected_committed: Vec<Entity>,
    pub(super) visible_levels: HashSet<i16>,
    pub(super) visible_by_authority: HashMap<Entity, Vec<CelestialClipmapCoverageCell>>,
    pub(super) live_authorities: HashSet<Entity>,
    pub(super) authority_stamps: HashMap<Entity, ClipmapAuthorityProjectionStamp>,
    pub(super) next_authority_stamps: HashMap<Entity, ClipmapAuthorityProjectionStamp>,
    pub(super) last_view_stamp: Option<ClipmapViewProjectionStamp>,
    pub(super) last_projection_epoch: u64,
    pub(super) last_frontier_epoch: u64,
    pub(super) initialized: bool,
    pub(super) pending_entities: Vec<Entity>,
}
