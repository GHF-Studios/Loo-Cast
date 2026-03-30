use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::config::{statics::CONFIG, types::ConfigValue};
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{
    USF_CONTENT_PACKAGES_BY_ID, USF_CONTENT_PROFILES_BY_ID, USF_DPT_SCHEMAS_BY_SCALE, USF_SCALE_BINDINGS_BY_SCALE, USF_ZONE_TYPES,
};
use crate::usf::definition::{DptMetricDefinition, DptMetricId, DptMetricStorageClass, DptMetricValueType, DptSchema, ZoneTypeId};
use crate::usf::scale::Scale;

pub const PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID: &str = "mod.placeholder_gameplay.v1";
pub const DPT_SAMPLER_KERNEL_DEFAULT_ID: &str = "dpt_sampler.kernel.default.v1";
pub const DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID: &str = "dpt_categorizer.kernel.zlm_lookup.v1";

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct ScaleContentBinding {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfActiveContentProfile {
    pub profile_id: String,
    pub configured_content_packages: Vec<UsfConfiguredContentPackage>,
    pub enabled_content_packages: HashSet<String>,
    pub resolved_enabled_content_packages: Vec<String>,
    pub bindings_by_scale: HashMap<Scale, ScaleContentBinding>,
    pub known_dpt_samplers: HashSet<String>,
    pub known_dpt_categorizers: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, DptSchema>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl Default for UsfActiveContentProfile {
    fn default() -> Self {
        let profile_id = active_usf_content_profile_id_from_config();
        let (configured_content_packages, resolved_enabled_content_packages) = configured_packages_for_profile(profile_id.as_str());
        let enabled_content_packages = configured_content_packages
            .iter()
            .filter(|package| package.enabled)
            .map(|package| package.content_package_id.clone())
            .collect::<HashSet<_>>();

        let mut profile = Self {
            profile_id,
            configured_content_packages,
            enabled_content_packages,
            resolved_enabled_content_packages,
            bindings_by_scale: HashMap::new(),
            known_dpt_samplers: script_dpt_samplers(),
            known_dpt_categorizers: script_dpt_categorizers(),
            schemas_by_scale: HashMap::new(),
            known_zone_types: HashSet::new(),
        };

        for (scale, binding) in script_scale_bindings() {
            profile.bindings_by_scale.insert(scale, binding);
        }
        for zone in script_zone_types() {
            profile.known_zone_types.insert(zone);
        }
        for (scale, schema) in script_schema_overrides() {
            profile.schemas_by_scale.insert(scale, schema);
        }

        if let Err(reason) = profile.validate() {
            panic!("USF active content profile default validation failed: {reason}");
        }

        profile
    }
}
impl UsfActiveContentProfile {
    pub fn schema_for_scale(&self, scale: Scale) -> Option<&DptSchema> {
        self.schemas_by_scale.get(&scale)
    }

    pub fn binding_for_scale(&self, scale: Scale) -> Option<&ScaleContentBinding> {
        self.bindings_by_scale.get(&scale)
    }

    pub fn enabled_content_packages_in_profile_order(&self) -> Vec<String> {
        self.resolved_enabled_content_packages.clone()
    }

    pub fn is_package_enabled(&self, content_package_id: &str) -> bool {
        self.enabled_content_packages.contains(content_package_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("USF active content profile id must not be empty".to_string());
        }
        if self.enabled_content_packages.is_empty() {
            return Err(format!(
                "USF active content profile '{}' resolved to zero enabled content packages",
                self.profile_id
            ));
        }
        if self.resolved_enabled_content_packages.is_empty() {
            return Err(format!(
                "USF active content profile '{}' resolved no ordered enabled content packages",
                self.profile_id
            ));
        }
        if self.configured_content_packages.is_empty() {
            return Err(format!("USF active content profile '{}' has no configured content packages", self.profile_id));
        }
        if self.schemas_by_scale.is_empty() {
            return Err(format!("USF active content profile '{}' has no DPT schemas", self.profile_id));
        }
        if self.bindings_by_scale.is_empty() {
            return Err(format!("USF active content profile '{}' has no scale bindings", self.profile_id));
        }
        if self.known_dpt_samplers.is_empty() {
            return Err(format!("USF active content profile '{}' has no known DPT samplers", self.profile_id));
        }
        if self.known_dpt_categorizers.is_empty() {
            return Err(format!("USF active content profile '{}' has no known DPT categorizers", self.profile_id));
        }
        let mut configured_package_ids = HashSet::<String>::new();
        for package in &self.configured_content_packages {
            if package.content_package_id.trim().is_empty() {
                return Err(format!("USF active content profile '{}' has configured package with empty id", self.profile_id));
            }
            if package.config_enabled_key.trim().is_empty() {
                return Err(format!(
                    "USF active content profile '{}' has package '{}' with empty config key",
                    self.profile_id, package.content_package_id
                ));
            }
            if !configured_package_ids.insert(package.content_package_id.clone()) {
                return Err(format!(
                    "USF active content profile '{}' has duplicate configured package '{}'",
                    self.profile_id, package.content_package_id
                ));
            }
        }
        for package_id in &self.enabled_content_packages {
            if !configured_package_ids.contains(package_id) {
                return Err(format!(
                    "USF active content profile '{}' marks unknown package '{}' as enabled",
                    self.profile_id, package_id
                ));
            }
        }
        for package_id in &self.resolved_enabled_content_packages {
            if !self.enabled_content_packages.contains(package_id) {
                return Err(format!(
                    "USF active content profile '{}' has ordered enabled package '{}' that is not in enabled_content_packages",
                    self.profile_id, package_id
                ));
            }
        }
        let resolved_enabled_set = self.resolved_enabled_content_packages.iter().cloned().collect::<HashSet<_>>();
        if resolved_enabled_set != self.enabled_content_packages {
            return Err(format!(
                "USF active content profile '{}' has mismatch between enabled_content_packages and resolved_enabled_content_packages",
                self.profile_id
            ));
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(binding) = self.bindings_by_scale.get(&scale) else {
                return Err(format!("missing scale binding for scale index {}", scale.index_from_top()));
            };
            if binding.dpt_sampler_id.trim().is_empty() {
                return Err(format!("empty dpt_sampler_id for scale index {}", scale.index_from_top()));
            }
            if binding.dpt_categorizer_id.trim().is_empty() {
                return Err(format!("empty dpt_categorizer_id for scale index {}", scale.index_from_top()));
            }
            if binding.chunk_store_key.trim().is_empty() {
                return Err(format!("empty chunk_store_key for scale index {}", scale.index_from_top()));
            }
            if !self.known_dpt_samplers.contains(&binding.dpt_sampler_id) {
                return Err(format!(
                    "unknown dpt_sampler_id '{}' for scale index {}",
                    binding.dpt_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !self.known_dpt_categorizers.contains(&binding.dpt_categorizer_id) {
                return Err(format!(
                    "unknown dpt_categorizer_id '{}' for scale index {}",
                    binding.dpt_categorizer_id,
                    scale.index_from_top()
                ));
            }
            let Some(schema) = self.schemas_by_scale.get(&scale) else {
                return Err(format!("missing DPT schema for scale index {}", scale.index_from_top()));
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

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfConfiguredContentPackage {
    pub content_package_id: String,
    pub default_enabled: bool,
    pub config_enabled_key: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UsfScriptContentProfileDefinition {
    pub content_package_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UsfScriptContentPackageDefinition {
    pub default_enabled: bool,
    pub config_enabled_key: String,
    pub priority: i32,
    pub dependencies: HashSet<String>,
    pub load_after: HashSet<String>,
    pub conflicts_with: HashSet<String>,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleExecutionRoute {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
    pub usf_content_profile_id: String,
    pub content_package_ids: Vec<String>,
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

fn script_dpt_samplers() -> HashSet<String> {
    HashSet::from([DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string()])
}

fn script_dpt_categorizers() -> HashSet<String> {
    HashSet::from([DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string()])
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn script_zone_types() -> Vec<ZoneTypeId> {
    let zone_types = USF_ZONE_TYPES().lock().unwrap().clone();
    let mut ordered = zone_types.into_iter().collect::<Vec<_>>();
    ordered.sort();
    ordered.into_iter().map(|zone_type| normalize_zone_type(&zone_type)).collect()
}

fn script_schema_overrides() -> Vec<(Scale, DptSchema)> {
    let schema_map = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clone();
    let mut ordered = schema_map.into_iter().collect::<Vec<_>>();
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
                    let value_type = DptMetricValueType::from_tag(&metric.value_type).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid value_type '{}'; expected one of: u8, u16, i32, f32, f64",
                            metric.name, metric.value_type
                        )
                    });
                    let storage_class = DptMetricStorageClass::from_tag(&metric.storage_class).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid storage_class '{}'; expected one of: uniform, brick",
                            metric.name, metric.storage_class
                        )
                    });

                    DptMetricDefinition {
                        id: DptMetricId(metric.id),
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
                DptSchema {
                    revision: script_schema.revision,
                    metrics,
                    fallback_zone: normalize_zone_type(&script_schema.fallback_zone),
                },
            ))
        })
        .collect()
}

fn script_scale_bindings() -> HashMap<Scale, ScaleContentBinding> {
    let bindings = USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().clone();
    let mut ordered = bindings.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, binding)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            Some((
                scale,
                ScaleContentBinding {
                    dpt_sampler_id: binding.dpt_sampler_id.trim().to_ascii_lowercase(),
                    dpt_categorizer_id: binding.dpt_categorizer_id.trim().to_ascii_lowercase(),
                    chunk_store_key: binding.chunk_store_key.trim().to_ascii_lowercase(),
                },
            ))
        })
        .collect()
}

fn script_content_packages() -> HashMap<String, UsfScriptContentPackageDefinition> {
    USF_CONTENT_PACKAGES_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(content_package_id, package)| {
            (
                content_package_id.clone(),
                UsfScriptContentPackageDefinition {
                    default_enabled: package.default_enabled,
                    config_enabled_key: package.config_enabled_key.trim().to_ascii_lowercase(),
                    priority: package.priority,
                    dependencies: package.dependencies.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                    load_after: package.load_after.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                    conflicts_with: package.conflicts_with.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                },
            )
        })
        .collect()
}

fn script_content_profiles() -> HashMap<String, UsfScriptContentProfileDefinition> {
    USF_CONTENT_PROFILES_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(profile_id, profile)| {
            (
                profile_id.clone(),
                UsfScriptContentProfileDefinition {
                    content_package_ids: profile
                        .content_package_ids
                        .iter()
                        .map(|content_package_id| content_package_id.trim().to_ascii_lowercase())
                        .collect(),
                },
            )
        })
        .collect()
}

fn active_usf_content_profile_id_from_config() -> String {
    match CONFIG().data.get("usf_content/active_modpack_id") {
        Some(ConfigValue::String(value)) => {
            let normalized = value.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                panic!("USF active content profile resolve failed: 'usf_content/active_modpack_id' must not be empty");
            }
            normalized
        }
        Some(other) => panic!(
            "USF active content profile resolve failed: 'usf_content/active_modpack_id' must be a string, got {:?}",
            other
        ),
        None => panic!("USF active content profile resolve failed: 'usf_content/active_modpack_id' must be configured explicitly"),
    }
}

fn configured_packages_for_profile(profile_id: &str) -> (Vec<UsfConfiguredContentPackage>, Vec<String>) {
    let profiles = script_content_profiles();
    let packages = script_content_packages();
    let Some(profile) = profiles.get(profile_id) else {
        panic!("USF active content profile resolve failed: profile '{}' is not registered", profile_id);
    };
    if profile.content_package_ids.is_empty() {
        panic!(
            "USF active content profile resolve failed: profile '{}' contains no content packages",
            profile_id
        );
    }

    let mut configured = Vec::<UsfConfiguredContentPackage>::new();
    let mut profile_index_by_package = HashMap::<String, usize>::new();
    let mut seen_package_ids = HashSet::<String>::new();
    for (index, content_package_id) in profile.content_package_ids.iter().enumerate() {
        if !seen_package_ids.insert(content_package_id.clone()) {
            panic!(
                "USF active content profile resolve failed: profile '{}' contains duplicate content package '{}'",
                profile_id, content_package_id
            );
        }
        profile_index_by_package.insert(content_package_id.clone(), index);
        let Some(package_definition) = packages.get(content_package_id) else {
            panic!(
                "USF active content profile resolve failed: profile '{}' references unknown content package '{}'",
                profile_id, content_package_id
            );
        };
        configured.push(UsfConfiguredContentPackage {
            content_package_id: content_package_id.clone(),
            default_enabled: package_definition.default_enabled,
            config_enabled_key: package_definition.config_enabled_key.clone(),
            enabled: content_package_enabled_from_config(content_package_id, package_definition),
        });
    }

    if configured.iter().all(|package| !package.enabled) {
        panic!(
            "USF active content profile resolve failed: profile '{}' resolves to zero enabled packages",
            profile_id
        );
    }

    let enabled_ids = configured
        .iter()
        .filter(|package| package.enabled)
        .map(|package| package.content_package_id.clone())
        .collect::<Vec<_>>();
    let enabled_set = enabled_ids.iter().cloned().collect::<HashSet<_>>();

    for package_id in &enabled_ids {
        let package_definition = packages.get(package_id).unwrap_or_else(|| {
            panic!(
                "USF active content profile resolve failed: package '{}' definition missing unexpectedly",
                package_id
            )
        });

        for dependency in &package_definition.dependencies {
            if !packages.contains_key(dependency) {
                panic!(
                    "USF active content profile resolve failed: package '{}' depends_on unknown package '{}'",
                    package_id, dependency
                );
            }
            if !enabled_set.contains(dependency) {
                panic!(
                    "USF active content profile resolve failed: package '{}' depends_on '{}' but dependency is not enabled in profile '{}'",
                    package_id, dependency, profile_id
                );
            }
        }

        for conflict in &package_definition.conflicts_with {
            if enabled_set.contains(conflict) {
                panic!(
                    "USF active content profile resolve failed: package '{}' conflicts_with '{}' and both are enabled in profile '{}'",
                    package_id, conflict, profile_id
                );
            }
        }
    }

    let mut indegree = HashMap::<String, usize>::new();
    let mut edges = HashMap::<String, HashSet<String>>::new();
    for package_id in &enabled_ids {
        indegree.insert(package_id.clone(), 0);
        edges.insert(package_id.clone(), HashSet::new());
    }

    for package_id in &enabled_ids {
        let package_definition = packages.get(package_id).unwrap_or_else(|| {
            panic!(
                "USF active content profile resolve failed: package '{}' definition missing unexpectedly",
                package_id
            )
        });

        for dependency in &package_definition.dependencies {
            if !enabled_set.contains(dependency) {
                continue;
            }
            let adjacency = edges
                .get_mut(dependency.as_str())
                .unwrap_or_else(|| panic!("USF active content profile resolve failed: graph missing dependency node '{}'", dependency));
            if adjacency.insert(package_id.clone()) {
                *indegree
                    .get_mut(package_id.as_str())
                    .unwrap_or_else(|| panic!("USF active content profile resolve failed: graph missing indegree for '{}'", package_id)) += 1;
            }
        }

        for after_package_id in &package_definition.load_after {
            if !packages.contains_key(after_package_id) {
                panic!(
                    "USF active content profile resolve failed: package '{}' load_after unknown package '{}'",
                    package_id, after_package_id
                );
            }
            if !enabled_set.contains(after_package_id) {
                continue;
            }
            let adjacency = edges.get_mut(after_package_id.as_str()).unwrap_or_else(|| {
                panic!(
                    "USF active content profile resolve failed: graph missing load_after node '{}'",
                    after_package_id
                )
            });
            if adjacency.insert(package_id.clone()) {
                *indegree
                    .get_mut(package_id.as_str())
                    .unwrap_or_else(|| panic!("USF active content profile resolve failed: graph missing indegree for '{}'", package_id)) += 1;
            }
        }
    }

    let mut resolved_enabled = Vec::<String>::new();
    let mut ready = indegree
        .iter()
        .filter_map(|(package_id, degree)| if *degree == 0 { Some(package_id.clone()) } else { None })
        .collect::<Vec<_>>();

    while !ready.is_empty() {
        ready.sort_by(|left, right| {
            let left_definition = packages.get(left).unwrap_or_else(|| {
                panic!(
                    "USF active content profile resolve failed: package '{}' missing during dependency resolution",
                    left
                )
            });
            let right_definition = packages.get(right).unwrap_or_else(|| {
                panic!(
                    "USF active content profile resolve failed: package '{}' missing during dependency resolution",
                    right
                )
            });
            right_definition
                .priority
                .cmp(&left_definition.priority)
                .then_with(|| {
                    profile_index_by_package
                        .get(left)
                        .copied()
                        .unwrap_or(usize::MAX)
                        .cmp(&profile_index_by_package.get(right).copied().unwrap_or(usize::MAX))
                })
                .then_with(|| left.cmp(right))
        });
        let current = ready.remove(0);
        resolved_enabled.push(current.clone());
        let outgoing = edges
            .get(current.as_str())
            .cloned()
            .unwrap_or_else(|| panic!("USF active content profile resolve failed: graph missing adjacency for package '{}'", current));
        for downstream in outgoing {
            let degree = indegree
                .get_mut(downstream.as_str())
                .unwrap_or_else(|| panic!("USF active content profile resolve failed: graph missing indegree for package '{}'", downstream));
            *degree = degree.saturating_sub(1);
            if *degree == 0 {
                ready.push(downstream);
            }
        }
    }

    if resolved_enabled.len() != enabled_ids.len() {
        let unresolved = indegree
            .iter()
            .filter_map(|(package_id, degree)| if *degree > 0 { Some(package_id.clone()) } else { None })
            .collect::<Vec<_>>();
        panic!(
            "USF active content profile resolve failed: dependency cycle detected in profile '{}'; unresolved packages: {:?}",
            profile_id, unresolved
        );
    }

    (configured, resolved_enabled)
}

fn content_package_enabled_from_config(content_package_id: &str, package: &UsfScriptContentPackageDefinition) -> bool {
    match CONFIG().data.get(package.config_enabled_key.as_str()) {
        Some(ConfigValue::Boolean(enabled)) => *enabled,
        Some(other) => panic!(
            "USF content package '{}' expected boolean config at '{}', got {:?}",
            content_package_id, package.config_enabled_key, other
        ),
        None => package.default_enabled,
    }
}

fn validate_usf_active_content_profile_system(active_profile: Res<UsfActiveContentProfile>) {
    if let Err(reason) = active_profile.validate() {
        panic!("USF active content profile validation failed: {reason}");
    }
}

fn rebuild_usf_execution_plan_system(mut execution_plan: ResMut<UsfExecutionPlan>, active_content_profile: Res<UsfActiveContentProfile>) {
    execution_plan.routes_by_scale.clear();
    let enabled_content_package_ids = active_content_profile.enabled_content_packages_in_profile_order();
    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let binding = active_content_profile.binding_for_scale(scale).unwrap_or_else(|| {
            panic!(
                "USF execution plan rebuild missing scale content binding for scale index {}",
                scale.index_from_top()
            )
        });
        if enabled_content_package_ids.is_empty() {
            panic!(
                "USF execution plan rebuild failed: active content profile '{}' has no enabled packages",
                active_content_profile.profile_id
            );
        }

        execution_plan.routes_by_scale.insert(
            scale,
            UsfScaleExecutionRoute {
                dpt_sampler_id: binding.dpt_sampler_id.clone(),
                dpt_categorizer_id: binding.dpt_categorizer_id.clone(),
                chunk_store_key: binding.chunk_store_key.clone(),
                usf_content_profile_id: active_content_profile.profile_id.clone(),
                content_package_ids: enabled_content_package_ids.clone(),
            },
        );
    }
}

fn validate_usf_execution_plan_system(execution_plan: Res<UsfExecutionPlan>, active_content_profile: Res<UsfActiveContentProfile>) {
    if active_content_profile.enabled_content_packages.is_empty() {
        panic!("USF execution plan validation failed: active content profile has no enabled packages");
    }

    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let Some(route) = execution_plan.route_for_scale(scale) else {
            panic!(
                "USF execution plan missing route for scale index {} (profile='{}')",
                scale.index_from_top(),
                active_content_profile.profile_id
            );
        };
        let Some(binding) = active_content_profile.binding_for_scale(scale) else {
            panic!(
                "USF execution plan validation failed: missing active-profile binding for scale {}",
                scale.index_from_top()
            );
        };
        if route.dpt_sampler_id != binding.dpt_sampler_id
            || route.dpt_categorizer_id != binding.dpt_categorizer_id
            || route.chunk_store_key != binding.chunk_store_key
        {
            panic!(
                "USF execution plan route mismatch at scale {} against active-profile binding",
                scale.index_from_top()
            );
        }
        if route.usf_content_profile_id != active_content_profile.profile_id {
            panic!(
                "USF execution plan route mismatch at scale {}: binding profile='{}', route profile='{}'",
                scale.index_from_top(),
                active_content_profile.profile_id,
                route.usf_content_profile_id
            );
        }
        if route.content_package_ids.is_empty() {
            panic!(
                "USF execution plan route has no content packages at scale {} for profile '{}'",
                scale.index_from_top(),
                route.usf_content_profile_id
            );
        }
        for content_package_id in &route.content_package_ids {
            if !active_content_profile.enabled_content_packages.contains(content_package_id) {
                panic!(
                    "USF execution plan route references package '{}' at scale {} \
                     but it is not enabled by active profile '{}'",
                    content_package_id,
                    scale.index_from_top(),
                    active_content_profile.profile_id
                );
            }
        }
    }
}

pub(crate) struct ContentPlugin;
impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfActiveContentProfile>().init_resource::<UsfExecutionPlan>().add_systems(
            Startup,
            (
                validate_usf_active_content_profile_system,
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

    fn scripted_scale_bindings_and_kernels() -> (HashMap<Scale, ScaleContentBinding>, HashSet<String>, HashSet<String>) {
        let sampler_id = "dpt_sampler.test.default.v1".to_string();
        let categorizer_id = "dpt_categorizer.test.default.v1".to_string();
        let mut bindings_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            bindings_by_scale.insert(
                scale,
                ScaleContentBinding {
                    dpt_sampler_id: sampler_id.clone(),
                    dpt_categorizer_id: categorizer_id.clone(),
                    chunk_store_key: "chunk_store.test.default".to_string(),
                },
            );
        }

        (bindings_by_scale, HashSet::from([sampler_id]), HashSet::from([categorizer_id]))
    }

    fn scripted_active_content_profile() -> UsfActiveContentProfile {
        let mut known_zone_types = HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));
        let (bindings_by_scale, known_dpt_samplers, known_dpt_categorizers) = scripted_scale_bindings_and_kernels();

        let mut schemas_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let scale_index = scale.index_from_top();
            schemas_by_scale.insert(
                scale,
                DptSchema {
                    revision: 1,
                    metrics: vec![DptMetricDefinition {
                        id: DptMetricId(0),
                        name: "density".to_string(),
                        value_type: DptMetricValueType::F32,
                        semantics_tag: "matter.density.normalized".to_string(),
                        storage_class: DptMetricStorageClass::Brick,
                        derived: false,
                        min_scale_index: scale_index,
                        max_scale_index: scale_index,
                    }],
                    fallback_zone: ZoneTypeId::new("void"),
                },
            );
        }

        UsfActiveContentProfile {
            profile_id: "content_profile.test.default".to_string(),
            configured_content_packages: vec![UsfConfiguredContentPackage {
                content_package_id: "content_package.test.default".to_string(),
                default_enabled: true,
                config_enabled_key: "usf_content/content_packages/test_default/enabled".to_string(),
                enabled: true,
            }],
            enabled_content_packages: HashSet::from(["content_package.test.default".to_string()]),
            resolved_enabled_content_packages: vec!["content_package.test.default".to_string()],
            bindings_by_scale,
            known_dpt_samplers,
            known_dpt_categorizers,
            schemas_by_scale,
            known_zone_types,
        }
    }

    #[test]
    fn active_content_profile_is_valid() {
        let active_profile = scripted_active_content_profile();
        assert!(active_profile.validate().is_ok());
    }

    #[test]
    fn active_content_profile_rejects_missing_scale_binding() {
        let mut active_profile = scripted_active_content_profile();
        active_profile.bindings_by_scale.remove(&Scale::MAX);
        let error = active_profile.validate().unwrap_err();
        assert!(error.contains("missing scale binding"));
    }

    #[test]
    fn active_content_profile_rejects_unknown_sampler_id() {
        let mut active_profile = scripted_active_content_profile();
        let binding = active_profile.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.dpt_sampler_id = "dpt_sampler.unknown".to_string();
        let error = active_profile.validate().unwrap_err();
        assert!(error.contains("unknown dpt_sampler_id"));
    }

    #[test]
    fn active_content_profile_rejects_missing_scale_schema() {
        let mut active_profile = scripted_active_content_profile();
        active_profile.schemas_by_scale.remove(&Scale::MAX);
        let error = active_profile.validate().unwrap_err();
        assert!(error.contains("missing DPT schema"));
    }

    #[test]
    fn active_content_profile_rejects_duplicate_configured_package_ids() {
        let mut active_profile = scripted_active_content_profile();
        active_profile.configured_content_packages.push(UsfConfiguredContentPackage {
            content_package_id: "content_package.test.default".to_string(),
            default_enabled: false,
            config_enabled_key: "usf_content/content_packages/test_default_dup/enabled".to_string(),
            enabled: true,
        });
        let error = active_profile.validate().unwrap_err();
        assert!(error.contains("duplicate configured package"));
    }

    #[test]
    fn active_content_profile_rejects_enabled_package_not_configured() {
        let mut active_profile = scripted_active_content_profile();
        active_profile.enabled_content_packages.insert("content_package.test.unknown".to_string());
        let error = active_profile.validate().unwrap_err();
        assert!(error.contains("marks unknown package"));
    }
}
