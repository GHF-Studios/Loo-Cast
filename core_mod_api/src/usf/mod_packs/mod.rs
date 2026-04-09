use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{ScriptUsfConceptCatalog, ScriptUsfModContribution, ScriptUsfModManifestDefinition, USF_CONCEPT_CATALOG};
use crate::usf::metric::{MetricDefinition, MetricId, MetricStorageClass, MetricValueType};
use crate::usf::metric_container::{MetricContainerLayout, is_sampler_kernel_id_supported};
use crate::usf::mods::UsfConfiguredMod;
use crate::usf::scale::Scale;
use crate::usf::zlm::is_categorizer_kernel_id_supported;
use crate::usf::zone::ZoneTypeId;

pub const DEFAULT_DEMO_MOD_ID: &str = "demo";
#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleDefinition {
    pub metric_sampler_id: String,
    pub metric_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfModDefinition {
    pub priority: i32,
    pub dependencies: HashSet<String>,
    pub load_after: HashSet<String>,
    pub conflicts_with: HashSet<String>,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfModpackDefinition {
    pub mod_ids: Vec<String>,
}

#[derive(Resource, Debug, Clone)]
pub struct UsfConceptCatalog {
    pub raw: ScriptUsfConceptCatalog,
}
impl FromWorld for UsfConceptCatalog {
    fn from_world(_world: &mut World) -> Self {
        Self {
            raw: USF_CONCEPT_CATALOG().lock().unwrap().clone(),
        }
    }
}
impl UsfConceptCatalog {
    pub fn snapshot(&self) -> ScriptUsfConceptCatalog {
        self.raw.clone()
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UsfModManifestRegistry {
    pub manifests_by_mod_id: HashMap<String, ScriptUsfModManifestDefinition>,
}
impl FromWorld for UsfModManifestRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF mod manifest registry bootstrap failed: missing UsfConceptCatalog resource."));

        let manifests_by_mod_id = catalog
            .mod_manifests_by_id
            .into_iter()
            .map(|(mod_id, manifest)| (mod_id.trim().to_ascii_lowercase(), manifest))
            .collect::<HashMap<_, _>>();

        Self { manifests_by_mod_id }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UsfModContributionRegistry {
    pub contributions_by_mod_id: HashMap<String, ScriptUsfModContribution>,
    pub composed: ScriptUsfModContribution,
}
impl FromWorld for UsfModContributionRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF mod contribution registry bootstrap failed: missing UsfConceptCatalog resource."));

        let contributions_by_mod_id = catalog
            .mod_contributions_by_id
            .into_iter()
            .map(|(mod_id, contribution)| (mod_id.trim().to_ascii_lowercase(), contribution))
            .collect::<HashMap<_, _>>();

        Self {
            contributions_by_mod_id,
            composed: catalog.composed,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfModRegistry {
    pub active_modpack_id: String,
    pub resolved_mod_ids: Vec<String>,
    pub mods_by_id: HashMap<String, UsfModDefinition>,
}
impl UsfModRegistry {
    pub fn get(&self, mod_id: &str) -> Option<&UsfModDefinition> {
        self.mods_by_id.get(mod_id)
    }
}
impl FromWorld for UsfModRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF mod registry bootstrap failed: missing UsfConceptCatalog resource."));
        Self {
            active_modpack_id: script_active_modpack_id(&catalog),
            resolved_mod_ids: script_resolved_mod_ids(&catalog),
            mods_by_id: script_mods(&catalog),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfModpackRegistry {
    pub active_modpack_id: String,
    pub modpacks_by_id: HashMap<String, UsfModpackDefinition>,
}
impl UsfModpackRegistry {
    pub fn active(&self) -> Option<&UsfModpackDefinition> {
        self.modpacks_by_id.get(self.active_modpack_id.as_str())
    }
}
impl FromWorld for UsfModpackRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF modpack registry bootstrap failed: missing UsfConceptCatalog resource."));
        Self {
            active_modpack_id: script_active_modpack_id(&catalog),
            modpacks_by_id: script_modpacks(&catalog),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfMetricRegistry {
    pub metrics_by_name: HashMap<String, MetricDefinition>,
    pub metric_name_by_id: HashMap<MetricId, String>,
}
impl UsfMetricRegistry {
    pub fn get_by_name(&self, name: &str) -> Option<&MetricDefinition> {
        self.metrics_by_name.get(name)
    }
}
impl FromWorld for UsfMetricRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF metric registry bootstrap failed: missing UsfConceptCatalog resource."));
        let metrics_by_name = script_metrics_by_name(&catalog);
        let metric_name_by_id = metrics_by_name
            .iter()
            .map(|(name, definition)| (definition.id, name.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            metrics_by_name,
            metric_name_by_id,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfMetricSetRegistry {
    pub metric_names_by_set_id: HashMap<String, Vec<String>>,
}
impl FromWorld for UsfMetricSetRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF metric set registry bootstrap failed: missing UsfConceptCatalog resource."));
        Self {
            metric_names_by_set_id: script_metric_sets(&catalog),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfScaleRegistry {
    pub scales_by_index: HashMap<Scale, UsfScaleDefinition>,
    pub known_metric_samplers: HashSet<String>,
    pub known_metric_categorizers: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, MetricContainerLayout>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl FromWorld for UsfScaleRegistry {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF scale registry bootstrap failed: missing UsfConceptCatalog resource."));
        let mut known_zone_types = HashSet::<ZoneTypeId>::new();
        for zone in script_zone_types(&catalog) {
            known_zone_types.insert(zone);
        }
        let mut schemas_by_scale = HashMap::<Scale, MetricContainerLayout>::new();
        for (scale, schema) in script_schema_overrides(&catalog) {
            schemas_by_scale.insert(scale, schema);
        }
        Self {
            scales_by_index: script_scales(&catalog),
            known_metric_samplers: script_metric_samplers(&catalog),
            known_metric_categorizers: script_metric_categorizers(&catalog),
            schemas_by_scale,
            known_zone_types,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfActiveModPack {
    pub mod_pack_id: String,
    pub configured_mods: Vec<UsfConfiguredMod>,
    pub enabled_mods: HashSet<String>,
    pub resolved_enabled_mods: Vec<String>,
    pub scales_by_index: HashMap<Scale, UsfScaleDefinition>,
    pub known_metric_samplers: HashSet<String>,
    pub known_metric_categorizers: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, MetricContainerLayout>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl FromWorld for UsfActiveModPack {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF active modpack bootstrap failed: missing UsfConceptCatalog resource."));
        let (mod_pack_id, configured_mods, resolved_enabled_mods) = configured_mods_for_active_modpack_from_catalog(&catalog);
        let enabled_mods = configured_mods.iter().map(|mod_entry| mod_entry.mod_id.clone()).collect::<HashSet<_>>();

        let mut active_modpack = Self {
            mod_pack_id,
            configured_mods,
            enabled_mods,
            resolved_enabled_mods,
            scales_by_index: HashMap::new(),
            known_metric_samplers: script_metric_samplers(&catalog),
            known_metric_categorizers: script_metric_categorizers(&catalog),
            schemas_by_scale: HashMap::new(),
            known_zone_types: HashSet::new(),
        };

        for (scale, definition) in script_scales(&catalog) {
            active_modpack.scales_by_index.insert(scale, definition);
        }
        for zone in script_zone_types(&catalog) {
            active_modpack.known_zone_types.insert(zone);
        }
        for (scale, schema) in script_schema_overrides(&catalog) {
            active_modpack.schemas_by_scale.insert(scale, schema);
        }

        if let Err(reason) = active_modpack.validate() {
            panic!("USF active modpack default validation failed: {reason}");
        }

        active_modpack
    }
}
impl UsfActiveModPack {
    pub fn schema_for_scale(&self, scale: Scale) -> Option<&MetricContainerLayout> {
        self.schemas_by_scale.get(&scale)
    }

    pub fn scale_definition_for_scale(&self, scale: Scale) -> Option<&UsfScaleDefinition> {
        self.scales_by_index.get(&scale)
    }

    pub fn enabled_mods_in_profile_order(&self) -> Vec<String> {
        self.resolved_enabled_mods.clone()
    }

    pub fn is_mod_enabled(&self, mod_id: &str) -> bool {
        self.enabled_mods.contains(mod_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.mod_pack_id.trim().is_empty() {
            return Err("USF active modpack id must not be empty".to_string());
        }
        if self.enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved to zero enabled mods", self.mod_pack_id));
        }
        if self.resolved_enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved no ordered enabled mods", self.mod_pack_id));
        }
        if self.configured_mods.is_empty() {
            return Err(format!("USF active modpack '{}' has no configured mods", self.mod_pack_id));
        }
        if self.schemas_by_scale.is_empty() {
            return Err(format!("USF active modpack '{}' has no metric container layouts", self.mod_pack_id));
        }
        if self.scales_by_index.is_empty() {
            return Err(format!("USF active modpack '{}' has no scale definitions", self.mod_pack_id));
        }
        if self.known_metric_samplers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known metric samplers", self.mod_pack_id));
        }
        if self.known_metric_categorizers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known metric categorizers", self.mod_pack_id));
        }
        let mut configured_mod_ids = HashSet::<String>::new();
        for mod_entry in &self.configured_mods {
            if mod_entry.mod_id.trim().is_empty() {
                return Err(format!("USF active modpack '{}' has configured mod with empty id", self.mod_pack_id));
            }
            if !configured_mod_ids.insert(mod_entry.mod_id.clone()) {
                return Err(format!(
                    "USF active modpack '{}' has duplicate configured mod '{}'",
                    self.mod_pack_id, mod_entry.mod_id
                ));
            }
        }
        for mod_id in &self.enabled_mods {
            if !configured_mod_ids.contains(mod_id) {
                return Err(format!("USF active modpack '{}' marks unknown mod '{}' as enabled", self.mod_pack_id, mod_id));
            }
        }
        for mod_id in &self.resolved_enabled_mods {
            if !self.enabled_mods.contains(mod_id) {
                return Err(format!(
                    "USF active modpack '{}' has ordered enabled mod '{}' that is not in enabled_mods",
                    self.mod_pack_id, mod_id
                ));
            }
        }
        let resolved_enabled_set = self.resolved_enabled_mods.iter().cloned().collect::<HashSet<_>>();
        if resolved_enabled_set != self.enabled_mods {
            return Err(format!(
                "USF active modpack '{}' has mismatch between enabled_mods and resolved_enabled_mods",
                self.mod_pack_id
            ));
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(scale_definition) = self.scales_by_index.get(&scale) else {
                return Err(format!("missing scale definition for scale index {}", scale.index_from_top()));
            };
            if scale_definition.metric_sampler_id.trim().is_empty() {
                return Err(format!("empty metric_sampler_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.metric_categorizer_id.trim().is_empty() {
                return Err(format!("empty metric_categorizer_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.chunk_store_key.trim().is_empty() {
                return Err(format!("empty chunk_store_key for scale index {}", scale.index_from_top()));
            }
            if !self.known_metric_samplers.contains(&scale_definition.metric_sampler_id) {
                return Err(format!(
                    "unknown metric_sampler_id '{}' for scale index {}",
                    scale_definition.metric_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !self.known_metric_categorizers.contains(&scale_definition.metric_categorizer_id) {
                return Err(format!(
                    "unknown metric_categorizer_id '{}' for scale index {}",
                    scale_definition.metric_categorizer_id,
                    scale.index_from_top()
                ));
            }
            if !is_sampler_kernel_id_supported(scale_definition.metric_sampler_id.as_str()) {
                return Err(format!(
                    "metric_sampler_id '{}' for scale index {} is registered but not runtime-supported",
                    scale_definition.metric_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !is_categorizer_kernel_id_supported(scale_definition.metric_categorizer_id.as_str()) {
                return Err(format!(
                    "metric_categorizer_id '{}' for scale index {} is registered but not runtime-supported",
                    scale_definition.metric_categorizer_id,
                    scale.index_from_top()
                ));
            }
            let Some(schema) = self.schemas_by_scale.get(&scale) else {
                return Err(format!("missing metric container layout for scale index {}", scale.index_from_top()));
            };
            schema.validate()?;
            for metric in &schema.metrics {
                if !metric.applies_to_scale(scale) {
                    return Err(format!(
                        "metric '{}' (id={}) is not valid for scale index {} (range=[{}..{}])",
                        metric.name,
                        metric.id.0,
                        scale.index_from_top(),
                        metric.min_scale_index,
                        metric.max_scale_index
                    ));
                }
            }
            if !self.known_zone_types.contains(&schema.fallback_zone) {
                return Err(format!(
                    "fallback zone '{}' for scale {} is not declared in known_zone_types",
                    schema.fallback_zone.0,
                    scale.index_from_top()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UsfRuntimeConceptView {
    catalog: ScriptUsfConceptCatalog,
    active_modpack: UsfActiveModPack,
    mod_registry: UsfModRegistry,
    modpack_registry: UsfModpackRegistry,
    metric_registry: UsfMetricRegistry,
    metric_set_registry: UsfMetricSetRegistry,
    scale_registry: UsfScaleRegistry,
    mod_manifest_registry: UsfModManifestRegistry,
    mod_contribution_registry: UsfModContributionRegistry,
}
impl FromWorld for UsfRuntimeConceptView {
    fn from_world(world: &mut World) -> Self {
        let catalog = world
            .get_resource::<UsfConceptCatalog>()
            .map(|catalog| catalog.snapshot())
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfConceptCatalog resource."));
        let active_modpack = world
            .get_resource::<UsfActiveModPack>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfActiveModPack resource."));
        let mod_registry = world
            .get_resource::<UsfModRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfModRegistry resource."));
        let modpack_registry = world
            .get_resource::<UsfModpackRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfModpackRegistry resource."));
        let metric_registry = world
            .get_resource::<UsfMetricRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfMetricRegistry resource."));
        let metric_set_registry = world
            .get_resource::<UsfMetricSetRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfMetricSetRegistry resource."));
        let scale_registry = world
            .get_resource::<UsfScaleRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfScaleRegistry resource."));
        let mod_manifest_registry = world
            .get_resource::<UsfModManifestRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfModManifestRegistry resource."));
        let mod_contribution_registry = world
            .get_resource::<UsfModContributionRegistry>()
            .cloned()
            .unwrap_or_else(|| panic!("USF runtime concept view bootstrap failed: missing UsfModContributionRegistry resource."));

        Self {
            catalog,
            active_modpack,
            mod_registry,
            modpack_registry,
            metric_registry,
            metric_set_registry,
            scale_registry,
            mod_manifest_registry,
            mod_contribution_registry,
        }
    }
}
impl UsfRuntimeConceptView {
    pub fn catalog_snapshot(&self) -> ScriptUsfConceptCatalog {
        self.catalog.clone()
    }

    pub fn active_modpack(&self) -> &UsfActiveModPack {
        &self.active_modpack
    }

    pub fn is_mod_enabled(&self, mod_id: &str) -> bool {
        self.active_modpack.is_mod_enabled(mod_id)
    }

    pub fn mod_definition(&self, mod_id: &str) -> Option<&UsfModDefinition> {
        self.mod_registry.mods_by_id.get(mod_id)
    }

    pub fn active_modpack_definition(&self) -> Option<&UsfModpackDefinition> {
        self.modpack_registry.active()
    }

    pub fn scale_definition_for_scale(&self, scale: Scale) -> Option<&UsfScaleDefinition> {
        self.scale_registry.scales_by_index.get(&scale)
    }

    pub fn schema_for_scale(&self, scale: Scale) -> Option<&MetricContainerLayout> {
        self.scale_registry.schemas_by_scale.get(&scale)
    }

    pub fn metric_definition(&self, metric_name: &str) -> Option<&MetricDefinition> {
        self.metric_registry.get_by_name(metric_name)
    }

    pub fn metric_set_members(&self, metric_set_id: &str) -> Option<&[String]> {
        self.metric_set_registry
            .metric_names_by_set_id
            .get(metric_set_id)
            .map(Vec::as_slice)
    }

    pub fn zone_types(&self) -> &HashSet<ZoneTypeId> {
        &self.scale_registry.known_zone_types
    }

    pub fn mod_manifest(&self, mod_id: &str) -> Option<&ScriptUsfModManifestDefinition> {
        self.mod_manifest_registry.manifests_by_mod_id.get(mod_id)
    }

    pub fn mod_contribution(&self, mod_id: &str) -> Option<&ScriptUsfModContribution> {
        self.mod_contribution_registry.contributions_by_mod_id.get(mod_id)
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleExecutionRoute {
    pub metric_sampler_id: String,
    pub metric_categorizer_id: String,
    pub chunk_store_key: String,
    pub usf_mod_pack_id: String,
    pub mod_ids: Vec<String>,
}

#[derive(Resource, Reflect, Debug, Clone, Default)]
#[reflect(Resource)]
pub struct UsfExecutionPlan {
    pub routes_by_scale: HashMap<Scale, UsfScaleExecutionRoute>,
}
impl UsfExecutionPlan {
    pub fn route_for_scale(&self, scale: Scale) -> Option<&UsfScaleExecutionRoute> {
        self.routes_by_scale.get(&scale)
    }
}

fn script_active_modpack_id(catalog: &ScriptUsfConceptCatalog) -> String {
    let active_modpack_id = catalog.active_modpack_id.trim().to_ascii_lowercase();
    if active_modpack_id.is_empty() {
        panic!("USF concept catalog resolve failed: active_modpack_id is empty");
    }
    active_modpack_id
}

fn script_resolved_mod_ids(catalog: &ScriptUsfConceptCatalog) -> Vec<String> {
    let resolved_mod_ids = catalog
        .resolved_mod_ids
        .iter()
        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
        .collect::<Vec<_>>();
    if resolved_mod_ids.is_empty() {
        panic!("USF concept catalog resolve failed: resolved_mod_ids is empty");
    }
    resolved_mod_ids
}

fn script_mods(catalog: &ScriptUsfConceptCatalog) -> HashMap<String, UsfModDefinition> {
    catalog
        .mods_by_id
        .iter()
        .map(|(mod_id, mod_definition)| {
            (
                mod_id.trim().to_ascii_lowercase(),
                UsfModDefinition {
                    priority: mod_definition.priority,
                    dependencies: mod_definition
                        .dependencies
                        .iter()
                        .map(|value| value.trim().to_ascii_lowercase())
                        .collect(),
                    load_after: mod_definition
                        .load_after
                        .iter()
                        .map(|value| value.trim().to_ascii_lowercase())
                        .collect(),
                    conflicts_with: mod_definition
                        .conflicts_with
                        .iter()
                        .map(|value| value.trim().to_ascii_lowercase())
                        .collect(),
                },
            )
        })
        .collect()
}

fn script_modpacks(catalog: &ScriptUsfConceptCatalog) -> HashMap<String, UsfModpackDefinition> {
    catalog
        .modpacks_by_id
        .iter()
        .map(|(modpack_id, modpack_definition)| {
            (
                modpack_id.trim().to_ascii_lowercase(),
                UsfModpackDefinition {
                    mod_ids: modpack_definition
                        .mod_ids
                        .iter()
                        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
                        .collect(),
                },
            )
        })
        .collect()
}

fn script_metrics_by_name(catalog: &ScriptUsfConceptCatalog) -> HashMap<String, MetricDefinition> {
    catalog
        .composed
        .metrics_by_name
        .iter()
        .map(|(metric_name, metric)| {
            let value_type = MetricValueType::from_tag(&metric.value_type).unwrap_or_else(|| {
                panic!(
                    "USF script metric '{}' has invalid value_type '{}'; expected one of: u8, u16, i32, f32, f64",
                    metric_name, metric.value_type
                )
            });
            let storage_class = MetricStorageClass::from_tag(&metric.storage_class).unwrap_or_else(|| {
                panic!(
                    "USF script metric '{}' has invalid storage_class '{}'; expected one of: uniform, brick",
                    metric_name, metric.storage_class
                )
            });
            (
                metric_name.trim().to_ascii_lowercase(),
                MetricDefinition {
                    id: MetricId(metric.id),
                    name: metric.name.trim().to_ascii_lowercase(),
                    value_type,
                    semantics_tag: metric.semantics_tag.trim().to_ascii_lowercase(),
                    storage_class,
                    derived: metric.derived,
                    min_scale_index: metric.min_scale_index,
                    max_scale_index: metric.max_scale_index,
                },
            )
        })
        .collect()
}

fn script_metric_sets(catalog: &ScriptUsfConceptCatalog) -> HashMap<String, Vec<String>> {
    catalog
        .composed
        .metric_sets_by_id
        .iter()
        .map(|(metric_set_id, metric_names)| {
            (
                metric_set_id.trim().to_ascii_lowercase(),
                metric_names
                    .iter()
                    .map(|metric_name| metric_name.trim().to_ascii_lowercase())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn configured_mods_for_active_modpack_from_catalog(catalog: &ScriptUsfConceptCatalog) -> (String, Vec<UsfConfiguredMod>, Vec<String>) {
    let mod_pack_id = catalog.active_modpack_id.trim().to_ascii_lowercase();
    if mod_pack_id.is_empty() {
        panic!("USF active modpack resolve failed: concept catalog has no active modpack id");
    }

    let Some(active_modpack) = catalog.modpacks_by_id.get(&mod_pack_id) else {
        panic!("USF active modpack resolve failed: modpack '{}' is not present in concept catalog", mod_pack_id);
    };
    if active_modpack.mod_ids.is_empty() {
        panic!("USF active modpack resolve failed: modpack '{}' contains no mods", mod_pack_id);
    }

    let configured_mods = active_modpack
        .mod_ids
        .iter()
        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
        .map(|mod_id| {
            if !catalog.mods_by_id.contains_key(&mod_id) {
                panic!(
                    "USF active modpack resolve failed: modpack '{}' references unknown mod '{}' in concept catalog",
                    mod_pack_id, mod_id
                );
            }
            UsfConfiguredMod { mod_id }
        })
        .collect::<Vec<_>>();

    let resolved_enabled_mods = catalog
        .resolved_mod_ids
        .iter()
        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
        .collect::<Vec<_>>();
    if resolved_enabled_mods.is_empty() {
        panic!(
            "USF active modpack resolve failed: concept catalog has no resolved enabled mods for modpack '{}'",
            mod_pack_id
        );
    }
    let configured_mod_set = configured_mods.iter().map(|entry| entry.mod_id.clone()).collect::<HashSet<_>>();
    for mod_id in &resolved_enabled_mods {
        if !configured_mod_set.contains(mod_id) {
            panic!(
                "USF active modpack resolve failed: resolved mod '{}' is not configured by modpack '{}'",
                mod_id, mod_pack_id
            );
        }
    }

    (mod_pack_id, configured_mods, resolved_enabled_mods)
}

fn script_metric_samplers(catalog: &ScriptUsfConceptCatalog) -> HashSet<String> {
    let kernels = catalog.composed.metric_sampler_kernel_ids.clone();
    if kernels.is_empty() {
        panic!("USF content bootstrap failed: no metric sampler kernels in concept catalog");
    }
    kernels
}

fn script_metric_categorizers(catalog: &ScriptUsfConceptCatalog) -> HashSet<String> {
    let kernels = catalog.composed.metric_categorizer_kernel_ids.clone();
    if kernels.is_empty() {
        panic!("USF content bootstrap failed: no metric categorizer kernels in concept catalog");
    }
    kernels
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn script_zone_types(catalog: &ScriptUsfConceptCatalog) -> Vec<ZoneTypeId> {
    let mut ordered = catalog.composed.zone_types.clone().into_iter().collect::<Vec<_>>();
    ordered.sort();
    ordered.into_iter().map(|zone_type| normalize_zone_type(&zone_type)).collect()
}

fn script_schema_overrides(catalog: &ScriptUsfConceptCatalog) -> Vec<(Scale, MetricContainerLayout)> {
    let mut ordered = catalog
        .composed
        .metric_container_layouts_by_scale
        .clone()
        .into_iter()
        .collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, script_schema)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            let metrics = script_schema
                .metrics
                .into_iter()
                .map(|metric| {
                    let value_type = MetricValueType::from_tag(&metric.value_type).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid value_type '{}'; expected one of: u8, u16, i32, f32, f64",
                            metric.name, metric.value_type
                        )
                    });
                    let storage_class = MetricStorageClass::from_tag(&metric.storage_class).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid storage_class '{}'; expected one of: uniform, brick",
                            metric.name, metric.storage_class
                        )
                    });

                    MetricDefinition {
                        id: MetricId(metric.id),
                        name: metric.name.trim().to_string(),
                        value_type,
                        semantics_tag: metric.semantics_tag.trim().to_string(),
                        storage_class,
                        derived: metric.derived,
                        min_scale_index: metric.min_scale_index,
                        max_scale_index: metric.max_scale_index,
                    }
                })
                .collect::<Vec<_>>();
            Some((
                scale,
                MetricContainerLayout {
                    revision: script_schema.revision,
                    metrics,
                    fallback_zone: normalize_zone_type(&script_schema.fallback_zone),
                },
            ))
        })
        .collect()
}

fn script_scales(catalog: &ScriptUsfConceptCatalog) -> HashMap<Scale, UsfScaleDefinition> {
    let mut ordered = catalog.composed.scales_by_index.clone().into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, binding)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            Some((
                scale,
                UsfScaleDefinition {
                    metric_sampler_id: binding.metric_sampler_id.trim().to_ascii_lowercase(),
                    metric_categorizer_id: binding.metric_categorizer_id.trim().to_ascii_lowercase(),
                    chunk_store_key: binding.chunk_store_key.trim().to_ascii_lowercase(),
                },
            ))
        })
        .collect()
}

fn validate_usf_concept_catalog_system(catalog: Res<UsfConceptCatalog>) {
    let snapshot = catalog.snapshot();
    if snapshot.active_modpack_id.trim().is_empty() {
        panic!("USF concept catalog validation failed: active_modpack_id is empty.");
    }
    if snapshot.resolved_mod_ids.is_empty() {
        panic!("USF concept catalog validation failed: resolved_mod_ids is empty.");
    }
    if snapshot.mods_by_id.is_empty() {
        panic!("USF concept catalog validation failed: mods_by_id is empty.");
    }
    if snapshot.modpacks_by_id.is_empty() {
        panic!("USF concept catalog validation failed: modpacks_by_id is empty.");
    }
}

fn validate_usf_mod_manifest_registry_system(mod_registry: Res<UsfModRegistry>, mod_manifest_registry: Res<UsfModManifestRegistry>) {
    if mod_manifest_registry.manifests_by_mod_id.is_empty() {
        panic!("USF mod manifest registry validation failed: no manifests are registered.");
    }
    for mod_id in mod_registry.mods_by_id.keys() {
        if !mod_manifest_registry.manifests_by_mod_id.contains_key(mod_id) {
            panic!(
                "USF mod manifest registry validation failed: missing manifest for mod '{}'.",
                mod_id
            );
        }
    }
}

fn validate_usf_mod_contribution_registry_system(
    mod_registry: Res<UsfModRegistry>,
    mod_manifest_registry: Res<UsfModManifestRegistry>,
    mod_contribution_registry: Res<UsfModContributionRegistry>,
) {
    if mod_contribution_registry.contributions_by_mod_id.is_empty() {
        panic!("USF mod contribution registry validation failed: no mod contributions are registered.");
    }

    for mod_id in &mod_registry.resolved_mod_ids {
        if !mod_contribution_registry.contributions_by_mod_id.contains_key(mod_id) {
            panic!(
                "USF mod contribution registry validation failed: missing contribution for resolved mod '{}'.",
                mod_id
            );
        }
    }

    for (mod_id, manifest) in &mod_manifest_registry.manifests_by_mod_id {
        let Some(contribution) = mod_contribution_registry.contributions_by_mod_id.get(mod_id) else {
            continue;
        };

        for required_metric in &manifest.required_metrics {
            if !contribution.metrics_by_name.contains_key(required_metric) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires metric '{}' but contribution does not define it.",
                    mod_id, required_metric
                );
            }
        }
        for required_metric_set in &manifest.required_metric_sets {
            if !contribution.metric_sets_by_id.contains_key(required_metric_set) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires metric_set '{}' but contribution does not define it.",
                    mod_id, required_metric_set
                );
            }
        }
        for required_zone_type in &manifest.required_zone_types {
            if !contribution.zone_types.contains(required_zone_type) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires zone '{}' but contribution does not define it.",
                    mod_id, required_zone_type
                );
            }
        }
        for required_phenomenon in &manifest.required_phenomena {
            if !contribution.phenomena_by_id.contains_key(required_phenomenon) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires phenomenon '{}' but contribution does not define it.",
                    mod_id, required_phenomenon
                );
            }
        }
        for required_model in &manifest.required_phenomenon_models {
            if !contribution.phenomenon_models_by_id.contains_key(required_model) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires phenomenon_model '{}' but contribution does not define it.",
                    mod_id, required_model
                );
            }
        }
        for required_scale in &manifest.required_scales {
            if !contribution.scales_by_index.contains_key(required_scale) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires scale {} but contribution does not define it.",
                    mod_id, required_scale
                );
            }
        }
        for required_schema_scale in &manifest.required_metric_container_layout_scales {
            if !contribution.metric_container_layouts_by_scale.contains_key(required_schema_scale) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires metric_container_layout scale {} but contribution does not define it.",
                    mod_id, required_schema_scale
                );
            }
        }
        for required_zlm_scale in &manifest.required_zlm_scales {
            if !contribution.zlm_scales_by_scale.contains_key(required_zlm_scale) {
                panic!(
                    "USF mod contribution registry validation failed: mod '{}' manifest requires zlm scale {} but contribution does not define it.",
                    mod_id, required_zlm_scale
                );
            }
        }
    }
}

fn validate_usf_mod_registry_system(mod_registry: Res<UsfModRegistry>, modpack_registry: Res<UsfModpackRegistry>) {
    if mod_registry.mods_by_id.is_empty() {
        panic!("USF mod registry validation failed: no mods are registered.");
    }
    if mod_registry.resolved_mod_ids.is_empty() {
        panic!("USF mod registry validation failed: resolved_mod_ids is empty.");
    }
    let Some(active_modpack) = modpack_registry.active() else {
        panic!(
            "USF mod registry validation failed: active modpack '{}' is not present in modpack registry.",
            modpack_registry.active_modpack_id
        );
    };
    if active_modpack.mod_ids.is_empty() {
        panic!(
            "USF mod registry validation failed: active modpack '{}' has no mods.",
            modpack_registry.active_modpack_id
        );
    }

    let mut seen = HashSet::<String>::new();
    for mod_id in &active_modpack.mod_ids {
        if !seen.insert(mod_id.clone()) {
            panic!(
                "USF mod registry validation failed: active modpack '{}' contains duplicate mod '{}'.",
                modpack_registry.active_modpack_id, mod_id
            );
        }
        if !mod_registry.mods_by_id.contains_key(mod_id) {
            panic!(
                "USF mod registry validation failed: active modpack '{}' references unknown mod '{}'.",
                modpack_registry.active_modpack_id, mod_id
            );
        }
    }

    for mod_id in &mod_registry.resolved_mod_ids {
        if !mod_registry.mods_by_id.contains_key(mod_id) {
            panic!(
                "USF mod registry validation failed: resolved mod '{}' is not present in mod registry.",
                mod_id
            );
        }
        if !seen.contains(mod_id) {
            panic!(
                "USF mod registry validation failed: resolved mod '{}' is not listed in active modpack '{}'.",
                mod_id, modpack_registry.active_modpack_id
            );
        }
    }
}

fn validate_usf_metric_registry_system(
    metric_registry: Res<UsfMetricRegistry>,
    metric_set_registry: Res<UsfMetricSetRegistry>,
    scale_registry: Res<UsfScaleRegistry>,
) {
    if metric_registry.metrics_by_name.is_empty() {
        panic!("USF metric registry validation failed: no metrics are registered.");
    }
    if metric_registry.metric_name_by_id.is_empty() {
        panic!("USF metric registry validation failed: metric_name_by_id is empty.");
    }
    for (metric_id, metric_name) in &metric_registry.metric_name_by_id {
        let Some(definition) = metric_registry.metrics_by_name.get(metric_name) else {
            panic!(
                "USF metric registry validation failed: metric id {} points to unknown metric '{}'.",
                metric_id.0, metric_name
            );
        };
        if definition.id != *metric_id {
            panic!(
                "USF metric registry validation failed: metric id mismatch for '{}': map has {}, definition has {}.",
                metric_name, metric_id.0, definition.id.0
            );
        }
    }

    if metric_set_registry.metric_names_by_set_id.is_empty() {
        panic!("USF metric set registry validation failed: no metric sets are registered.");
    }
    for (metric_set_id, metric_names) in &metric_set_registry.metric_names_by_set_id {
        if metric_names.is_empty() {
            panic!(
                "USF metric set registry validation failed: metric set '{}' has no metrics.",
                metric_set_id
            );
        }
        let mut seen = HashSet::<String>::new();
        for metric_name in metric_names {
            if !seen.insert(metric_name.clone()) {
                panic!(
                    "USF metric set registry validation failed: metric set '{}' contains duplicate metric '{}'.",
                    metric_set_id, metric_name
                );
            }
            if !metric_registry.metrics_by_name.contains_key(metric_name) {
                panic!(
                    "USF metric set registry validation failed: metric set '{}' references unknown metric '{}'.",
                    metric_set_id, metric_name
                );
            }
        }
    }

    for (scale, schema) in &scale_registry.schemas_by_scale {
        for schema_metric in &schema.metrics {
            let schema_metric_name = schema_metric.name.trim().to_ascii_lowercase();
            let Some(registry_metric) = metric_registry.metrics_by_name.get(&schema_metric_name) else {
                panic!(
                    "USF metric registry validation failed: schema metric '{}' at scale {} is not registered.",
                    schema_metric.name,
                    scale.index_from_top()
                );
            };
            if registry_metric.id != schema_metric.id {
                panic!(
                    "USF metric registry validation failed: schema metric '{}' id mismatch at scale {} (schema={}, registry={}).",
                    schema_metric.name,
                    scale.index_from_top(),
                    schema_metric.id.0,
                    registry_metric.id.0
                );
            }
        }
    }
}

fn validate_usf_scale_registry_system(scale_registry: Res<UsfScaleRegistry>) {
    if scale_registry.scales_by_index.is_empty() {
        panic!("USF scale registry validation failed: no scales are registered.");
    }
    if scale_registry.schemas_by_scale.is_empty() {
        panic!("USF scale registry validation failed: no metric container layouts are registered.");
    }
    if scale_registry.known_metric_samplers.is_empty() {
        panic!("USF scale registry validation failed: no metric samplers are registered.");
    }
    if scale_registry.known_metric_categorizers.is_empty() {
        panic!("USF scale registry validation failed: no metric categorizers are registered.");
    }
    if scale_registry.known_zone_types.is_empty() {
        panic!("USF scale registry validation failed: no zone types are registered.");
    }

    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let Some(scale_definition) = scale_registry.scales_by_index.get(&scale) else {
            panic!(
                "USF scale registry validation failed: missing scale definition for scale {}.",
                scale.index_from_top()
            );
        };
        if !scale_registry.known_metric_samplers.contains(&scale_definition.metric_sampler_id) {
            panic!(
                "USF scale registry validation failed: unknown metric sampler '{}' for scale {}.",
                scale_definition.metric_sampler_id,
                scale.index_from_top()
            );
        }
        if !scale_registry
            .known_metric_categorizers
            .contains(&scale_definition.metric_categorizer_id)
        {
            panic!(
                "USF scale registry validation failed: unknown metric categorizer '{}' for scale {}.",
                scale_definition.metric_categorizer_id,
                scale.index_from_top()
            );
        }
        if !is_sampler_kernel_id_supported(scale_definition.metric_sampler_id.as_str()) {
            panic!(
                "USF scale registry validation failed: sampler '{}' is not runtime-supported for scale {}.",
                scale_definition.metric_sampler_id,
                scale.index_from_top()
            );
        }
        if !is_categorizer_kernel_id_supported(scale_definition.metric_categorizer_id.as_str()) {
            panic!(
                "USF scale registry validation failed: categorizer '{}' is not runtime-supported for scale {}.",
                scale_definition.metric_categorizer_id,
                scale.index_from_top()
            );
        }
        let Some(schema) = scale_registry.schemas_by_scale.get(&scale) else {
            panic!(
                "USF scale registry validation failed: missing metric container layout for scale {}.",
                scale.index_from_top()
            );
        };
        if let Err(reason) = schema.validate() {
            panic!(
                "USF scale registry validation failed: invalid metric container layout for scale {}: {}",
                scale.index_from_top(),
                reason
            );
        }
        if !scale_registry.known_zone_types.contains(&schema.fallback_zone) {
            panic!(
                "USF scale registry validation failed: fallback zone '{}' for scale {} is not registered.",
                schema.fallback_zone.0,
                scale.index_from_top()
            );
        }
    }
}

fn validate_usf_active_mod_pack_alignment_system(
    active_modpack: Res<UsfActiveModPack>,
    mod_registry: Res<UsfModRegistry>,
    modpack_registry: Res<UsfModpackRegistry>,
    scale_registry: Res<UsfScaleRegistry>,
) {
    if active_modpack.mod_pack_id != modpack_registry.active_modpack_id {
        panic!(
            "USF active modpack alignment failed: active modpack id '{}' differs from modpack registry '{}'.",
            active_modpack.mod_pack_id, modpack_registry.active_modpack_id
        );
    }
    if active_modpack.mod_pack_id != mod_registry.active_modpack_id {
        panic!(
            "USF active modpack alignment failed: active modpack id '{}' differs from mod registry '{}'.",
            active_modpack.mod_pack_id, mod_registry.active_modpack_id
        );
    }
    let active_modpack_def = modpack_registry.active().unwrap_or_else(|| {
        panic!(
            "USF active modpack alignment failed: active modpack '{}' missing in modpack registry.",
            modpack_registry.active_modpack_id
        )
    });
    let active_configured = active_modpack
        .configured_mods
        .iter()
        .map(|entry| entry.mod_id.clone())
        .collect::<Vec<_>>();
    if active_configured != active_modpack_def.mod_ids {
        panic!("USF active modpack alignment failed: configured mod list diverges from active modpack registry.");
    }
    if active_modpack.resolved_enabled_mods != mod_registry.resolved_mod_ids {
        panic!("USF active modpack alignment failed: resolved enabled mod list diverges from mod registry.");
    }
    if active_modpack.scales_by_index != scale_registry.scales_by_index {
        panic!("USF active modpack alignment failed: scale definitions diverge from scale registry.");
    }
    if active_modpack.known_metric_samplers != scale_registry.known_metric_samplers {
        panic!("USF active modpack alignment failed: metric samplers diverge from scale registry.");
    }
    if active_modpack.known_metric_categorizers != scale_registry.known_metric_categorizers {
        panic!("USF active modpack alignment failed: metric categorizers diverge from scale registry.");
    }
    if active_modpack.schemas_by_scale != scale_registry.schemas_by_scale {
        panic!("USF active modpack alignment failed: schemas diverge from scale registry.");
    }
    if active_modpack.known_zone_types != scale_registry.known_zone_types {
        panic!("USF active modpack alignment failed: zone types diverge from scale registry.");
    }
}

fn validate_usf_active_mod_pack_system(active_modpack: Res<UsfActiveModPack>) {
    if let Err(reason) = active_modpack.validate() {
        panic!("USF active modpack validation failed: {reason}");
    }
}

fn validate_usf_runtime_concept_view_system(
    runtime_view: Res<UsfRuntimeConceptView>,
    active_modpack: Res<UsfActiveModPack>,
    mod_registry: Res<UsfModRegistry>,
    modpack_registry: Res<UsfModpackRegistry>,
    metric_registry: Res<UsfMetricRegistry>,
    metric_set_registry: Res<UsfMetricSetRegistry>,
    scale_registry: Res<UsfScaleRegistry>,
    mod_manifest_registry: Res<UsfModManifestRegistry>,
    mod_contribution_registry: Res<UsfModContributionRegistry>,
) {
    let catalog = runtime_view.catalog_snapshot();
    if catalog.active_modpack_id.trim().is_empty() {
        panic!("USF runtime concept view validation failed: catalog snapshot has empty active_modpack_id.");
    }
    if runtime_view.active_modpack().mod_pack_id != active_modpack.mod_pack_id {
        panic!("USF runtime concept view validation failed: active modpack id mismatch.");
    }
    if runtime_view.active_modpack().resolved_enabled_mods != active_modpack.resolved_enabled_mods {
        panic!("USF runtime concept view validation failed: resolved enabled mods mismatch.");
    }
    if runtime_view.mod_registry.mods_by_id != mod_registry.mods_by_id
        || runtime_view.mod_registry.resolved_mod_ids != mod_registry.resolved_mod_ids
        || runtime_view.modpack_registry.modpacks_by_id != modpack_registry.modpacks_by_id
        || runtime_view.metric_registry.metrics_by_name != metric_registry.metrics_by_name
        || runtime_view.metric_set_registry.metric_names_by_set_id != metric_set_registry.metric_names_by_set_id
        || runtime_view.scale_registry.scales_by_index != scale_registry.scales_by_index
        || runtime_view.scale_registry.schemas_by_scale != scale_registry.schemas_by_scale
        || runtime_view.scale_registry.known_zone_types != scale_registry.known_zone_types
    {
        panic!("USF runtime concept view validation failed: registry mirror mismatch.");
    }
    if runtime_view.mod_manifest_registry.manifests_by_mod_id.len() != mod_manifest_registry.manifests_by_mod_id.len() {
        panic!("USF runtime concept view validation failed: mod manifest registry size mismatch.");
    }
    if runtime_view.mod_contribution_registry.contributions_by_mod_id.len() != mod_contribution_registry.contributions_by_mod_id.len() {
        panic!("USF runtime concept view validation failed: mod contribution registry size mismatch.");
    }
}

fn rebuild_usf_execution_plan_system(mut execution_plan: ResMut<UsfExecutionPlan>, active_modpack: Res<UsfActiveModPack>) {
    execution_plan.routes_by_scale.clear();
    let enabled_mod_ids = active_modpack.enabled_mods_in_profile_order();
    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let scale_definition = active_modpack
            .scale_definition_for_scale(scale)
            .unwrap_or_else(|| panic!("USF execution plan rebuild missing scale definition for scale index {}", scale.index_from_top()));
        if enabled_mod_ids.is_empty() {
            panic!(
                "USF execution plan rebuild failed: active modpack '{}' has no enabled mods",
                active_modpack.mod_pack_id
            );
        }

        execution_plan.routes_by_scale.insert(
            scale,
            UsfScaleExecutionRoute {
                metric_sampler_id: scale_definition.metric_sampler_id.clone(),
                metric_categorizer_id: scale_definition.metric_categorizer_id.clone(),
                chunk_store_key: scale_definition.chunk_store_key.clone(),
                usf_mod_pack_id: active_modpack.mod_pack_id.clone(),
                mod_ids: enabled_mod_ids.clone(),
            },
        );
    }
}

fn validate_usf_execution_plan_system(execution_plan: Res<UsfExecutionPlan>, active_modpack: Res<UsfActiveModPack>) {
    if active_modpack.enabled_mods.is_empty() {
        panic!("USF execution plan validation failed: active modpack has no enabled mods");
    }

    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let Some(route) = execution_plan.route_for_scale(scale) else {
            panic!(
                "USF execution plan missing route for scale index {} (modpack='{}')",
                scale.index_from_top(),
                active_modpack.mod_pack_id
            );
        };
        let Some(scale_definition) = active_modpack.scale_definition_for_scale(scale) else {
            panic!(
                "USF execution plan validation failed: missing active modpack scale definition for scale {}",
                scale.index_from_top()
            );
        };
        if route.metric_sampler_id != scale_definition.metric_sampler_id
            || route.metric_categorizer_id != scale_definition.metric_categorizer_id
            || route.chunk_store_key != scale_definition.chunk_store_key
        {
            panic!(
                "USF execution plan route mismatch at scale {} against active modpack scale definition",
                scale.index_from_top()
            );
        }
        if route.usf_mod_pack_id != active_modpack.mod_pack_id {
            panic!(
                "USF execution plan route mismatch at scale {}: active modpack='{}', route modpack='{}'",
                scale.index_from_top(),
                active_modpack.mod_pack_id,
                route.usf_mod_pack_id
            );
        }
        if route.mod_ids.is_empty() {
            panic!(
                "USF execution plan route has no mods at scale {} for modpack '{}'",
                scale.index_from_top(),
                route.usf_mod_pack_id
            );
        }
        for mod_id in &route.mod_ids {
            if !active_modpack.enabled_mods.contains(mod_id) {
                panic!(
                    "USF execution plan route references mod '{}' at scale {} \
                     but it is not enabled by active modpack '{}'",
                    mod_id,
                    scale.index_from_top(),
                    active_modpack.mod_pack_id
                );
            }
        }
    }
}

pub(crate) struct ModPacksPlugin;
impl Plugin for ModPacksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfConceptCatalog>()
            .init_resource::<UsfModManifestRegistry>()
            .init_resource::<UsfModContributionRegistry>()
            .init_resource::<UsfModRegistry>()
            .init_resource::<UsfModpackRegistry>()
            .init_resource::<UsfMetricRegistry>()
            .init_resource::<UsfMetricSetRegistry>()
            .init_resource::<UsfScaleRegistry>()
            .init_resource::<UsfActiveModPack>()
            .init_resource::<UsfRuntimeConceptView>()
            .init_resource::<UsfExecutionPlan>()
            .add_systems(
                Startup,
                (
                    validate_usf_concept_catalog_system,
                    validate_usf_mod_registry_system,
                    validate_usf_mod_manifest_registry_system,
                    validate_usf_mod_contribution_registry_system,
                    validate_usf_scale_registry_system,
                    validate_usf_metric_registry_system,
                    validate_usf_active_mod_pack_alignment_system,
                    validate_usf_active_mod_pack_system,
                    validate_usf_runtime_concept_view_system,
                    rebuild_usf_execution_plan_system,
                    validate_usf_execution_plan_system,
                )
                    .chain()
                    .in_set(AppSet::Diagnostics),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scripted_scales_and_kernels() -> (HashMap<Scale, UsfScaleDefinition>, HashSet<String>, HashSet<String>) {
        let sampler_id = crate::usf::metric_container::METRIC_SAMPLER_KERNEL_DEFAULT_ID.to_string();
        let categorizer_id = crate::usf::zlm::METRIC_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string();
        let mut scales_by_index = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            scales_by_index.insert(
                scale,
                UsfScaleDefinition {
                    metric_sampler_id: sampler_id.clone(),
                    metric_categorizer_id: categorizer_id.clone(),
                    chunk_store_key: "chunk_store.test.default".to_string(),
                },
            );
        }

        (scales_by_index, HashSet::from([sampler_id]), HashSet::from([categorizer_id]))
    }

    fn scripted_active_modpack() -> UsfActiveModPack {
        let mut known_zone_types = HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));
        let (scales_by_index, known_metric_samplers, known_metric_categorizers) = scripted_scales_and_kernels();

        let mut schemas_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let scale_index = scale.index_from_top();
            schemas_by_scale.insert(
                scale,
                MetricContainerLayout {
                    revision: 1,
                    metrics: vec![MetricDefinition {
                        id: MetricId(0),
                        name: "density".to_string(),
                        value_type: MetricValueType::F32,
                        semantics_tag: "matter.density.normalized".to_string(),
                        storage_class: MetricStorageClass::Brick,
                        derived: false,
                        min_scale_index: scale_index,
                        max_scale_index: scale_index,
                    }],
                    fallback_zone: ZoneTypeId::new("void"),
                },
            );
        }

        UsfActiveModPack {
            mod_pack_id: "modpack.test.default".to_string(),
            configured_mods: vec![UsfConfiguredMod {
                mod_id: "mod.test.default".to_string(),
            }],
            enabled_mods: HashSet::from(["mod.test.default".to_string()]),
            resolved_enabled_mods: vec!["mod.test.default".to_string()],
            scales_by_index,
            known_metric_samplers,
            known_metric_categorizers,
            schemas_by_scale,
            known_zone_types,
        }
    }

    #[test]
    fn active_modpack_is_valid() {
        let active_modpack = scripted_active_modpack();
        assert!(active_modpack.validate().is_ok());
    }

    #[test]
    fn active_modpack_rejects_missing_scale_definition() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.scales_by_index.remove(&Scale::MAX);
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("missing scale definition"));
    }

    #[test]
    fn active_modpack_rejects_unknown_sampler_id() {
        let mut active_modpack = scripted_active_modpack();
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_sampler_id = "metric_sampler.unknown".to_string();
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("unknown metric_sampler_id"));
    }

    #[test]
    fn active_modpack_rejects_registered_but_runtime_unsupported_sampler_id() {
        let mut active_modpack = scripted_active_modpack();
        let unsupported = "metric_sampler.custom_legacy_id".to_string();
        active_modpack.known_metric_samplers.insert(unsupported.clone());
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_sampler_id = unsupported;
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("not runtime-supported"));
    }

    #[test]
    fn active_modpack_rejects_registered_but_runtime_unsupported_categorizer_id() {
        let mut active_modpack = scripted_active_modpack();
        let unsupported = "metric_categorizer.custom_legacy_id".to_string();
        active_modpack.known_metric_categorizers.insert(unsupported.clone());
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_categorizer_id = unsupported;
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("not runtime-supported"));
    }

    #[test]
    fn active_modpack_accepts_runtime_supported_custom_kernel_ids() {
        let mut active_modpack = scripted_active_modpack();
        let custom_sampler = "metric_sampler.kernel.scale_max.experimental.v1".to_string();
        let custom_categorizer = "metric_categorizer.kernel.scale_max.experimental.v1".to_string();
        active_modpack.known_metric_samplers.insert(custom_sampler.clone());
        active_modpack.known_metric_categorizers.insert(custom_categorizer.clone());

        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_sampler_id = custom_sampler;
        scale_definition.metric_categorizer_id = custom_categorizer;

        let validation = active_modpack.validate();
        assert!(validation.is_ok(), "expected custom runtime-supported kernel ids to validate, got: {validation:?}");
    }

    #[test]
    fn active_modpack_rejects_missing_scale_schema() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.schemas_by_scale.remove(&Scale::MAX);
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("missing metric container layout"));
    }

    #[test]
    fn active_modpack_rejects_duplicate_configured_mod_ids() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.configured_mods.push(UsfConfiguredMod {
            mod_id: "mod.test.default".to_string(),
        });
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("duplicate configured mod"));
    }

    #[test]
    fn active_modpack_rejects_enabled_mod_not_configured() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.enabled_mods.insert("mod.test.unknown".to_string());
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("marks unknown mod"));
    }
}
