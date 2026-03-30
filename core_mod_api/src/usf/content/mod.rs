use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::config::{statics::CONFIG, types::ConfigValue};
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{
    USF_CONTENT_PACKAGES_BY_ID, USF_CONTENT_PROFILES_BY_ID, USF_DPT_SCHEMAS_BY_SCALE, USF_SCALE_BINDINGS_BY_SCALE, USF_ZONE_TYPES,
};
use crate::usf::definition::{DptMetricDefinition, DptMetricId, DptMetricStorageClass, DptMetricValueType, DptSchema, ZoneTypeId};
use crate::usf::scale::Scale;

pub const PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID: &str = "content_package.placeholder_gameplay.v1";
pub const DPT_SAMPLER_KERNEL_DEFAULT_ID: &str = "dpt_sampler.kernel.default.v1";
pub const DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID: &str = "dpt_categorizer.kernel.zlm_lookup.v1";

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct ScaleContentBinding {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
    pub usf_content_profile_id: String,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct ScaleContentRegistry {
    pub bindings_by_scale: HashMap<Scale, ScaleContentBinding>,
    pub known_dpt_samplers: HashSet<String>,
    pub known_dpt_categorizers: HashSet<String>,
}
impl Default for ScaleContentRegistry {
    fn default() -> Self {
        let registry = Self {
            bindings_by_scale: script_scale_bindings(),
            known_dpt_samplers: script_dpt_samplers(),
            known_dpt_categorizers: script_dpt_categorizers(),
        };

        if let Err(reason) = registry.validate() {
            panic!("USF scale content registry default validation failed: {reason}");
        }

        registry
    }
}
impl ScaleContentRegistry {
    pub fn binding_for_scale(&self, scale: Scale) -> Option<&ScaleContentBinding> {
        self.bindings_by_scale.get(&scale)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.known_dpt_samplers.is_empty() {
            return Err("no DPT samplers registered".to_string());
        }
        if self.known_dpt_categorizers.is_empty() {
            return Err("no DPT categorizers registered".to_string());
        }
        if self.bindings_by_scale.is_empty() {
            return Err("no scale content bindings registered".to_string());
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(binding) = self.bindings_by_scale.get(&scale) else {
                return Err(format!("missing scale content binding for scale index {}", scale.index_from_top()));
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
            if binding.usf_content_profile_id.trim().is_empty() {
                return Err(format!("empty usf_content_profile_id for scale index {}", scale.index_from_top()));
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
        }

        Ok(())
    }
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfActiveContentProfile {
    pub profile_id: String,
    pub configured_content_packages: Vec<UsfConfiguredContentPackage>,
    pub enabled_content_packages: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, DptSchema>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl Default for UsfActiveContentProfile {
    fn default() -> Self {
        let profile_id = active_usf_content_profile_id_from_config();
        let configured_content_packages = configured_packages_for_profile(profile_id.as_str());
        let enabled_content_packages = configured_content_packages
            .iter()
            .filter(|package| package.enabled)
            .map(|package| package.content_package_id.clone())
            .collect::<HashSet<_>>();

        let mut profile = Self {
            profile_id,
            configured_content_packages,
            enabled_content_packages,
            schemas_by_scale: HashMap::new(),
            known_zone_types: HashSet::new(),
        };

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

    pub fn enabled_content_packages_in_profile_order(&self) -> Vec<String> {
        self.configured_content_packages
            .iter()
            .filter(|package| package.enabled)
            .map(|package| package.content_package_id.clone())
            .collect()
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
        if self.configured_content_packages.is_empty() {
            return Err(format!("USF active content profile '{}' has no configured content packages", self.profile_id));
        }
        if self.schemas_by_scale.is_empty() {
            return Err(format!("USF active content profile '{}' has no DPT schemas", self.profile_id));
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

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
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
                    usf_content_profile_id: binding.usf_content_profile_id.trim().to_ascii_lowercase(),
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
    match CONFIG().data.get("usf_content/active_profile_id") {
        Some(ConfigValue::String(value)) => {
            let normalized = value.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                panic!("USF active content profile resolve failed: 'usf_content/active_profile_id' must not be empty");
            }
            normalized
        }
        Some(other) => panic!(
            "USF active content profile resolve failed: 'usf_content/active_profile_id' must be a string, got {:?}",
            other
        ),
        None => panic!("USF active content profile resolve failed: 'usf_content/active_profile_id' must be configured explicitly"),
    }
}

fn configured_packages_for_profile(profile_id: &str) -> Vec<UsfConfiguredContentPackage> {
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
    let mut seen_package_ids = HashSet::<String>::new();
    for content_package_id in &profile.content_package_ids {
        if !seen_package_ids.insert(content_package_id.clone()) {
            panic!(
                "USF active content profile resolve failed: profile '{}' contains duplicate content package '{}'",
                profile_id, content_package_id
            );
        }
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

    configured
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

fn validate_scale_content_registry_system(registry: Res<ScaleContentRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF scale content registry validation failed: {reason}");
    }
}

fn validate_usf_active_content_profile_system(active_profile: Res<UsfActiveContentProfile>) {
    if let Err(reason) = active_profile.validate() {
        panic!("USF active content profile validation failed: {reason}");
    }
}

fn rebuild_usf_execution_plan_system(
    mut execution_plan: ResMut<UsfExecutionPlan>,
    scale_content_registry: Res<ScaleContentRegistry>,
    active_content_profile: Res<UsfActiveContentProfile>,
) {
    execution_plan.routes_by_scale.clear();
    let enabled_content_package_ids = active_content_profile.enabled_content_packages_in_profile_order();
    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let binding = scale_content_registry.binding_for_scale(scale).unwrap_or_else(|| {
            panic!(
                "USF execution plan rebuild missing scale content binding for scale index {}",
                scale.index_from_top()
            )
        });
        if binding.usf_content_profile_id != active_content_profile.profile_id {
            panic!(
                "USF execution plan rebuild profile mismatch at scale {}: \
                 binding references '{}', but active profile is '{}'",
                scale.index_from_top(),
                binding.usf_content_profile_id,
                active_content_profile.profile_id
            );
        }
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
                usf_content_profile_id: binding.usf_content_profile_id.clone(),
                content_package_ids: enabled_content_package_ids.clone(),
            },
        );
    }
}

fn validate_usf_execution_plan_system(
    execution_plan: Res<UsfExecutionPlan>,
    scale_content_registry: Res<ScaleContentRegistry>,
    active_content_profile: Res<UsfActiveContentProfile>,
) {
    if active_content_profile.enabled_content_packages.is_empty() {
        panic!("USF execution plan validation failed: active content profile has no enabled packages");
    }

    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let Some(binding) = scale_content_registry.binding_for_scale(scale) else {
            continue;
        };
        let Some(route) = execution_plan.route_for_scale(scale) else {
            panic!(
                "USF execution plan missing route for scale index {} (profile='{}')",
                scale.index_from_top(),
                binding.usf_content_profile_id
            );
        };
        if route.usf_content_profile_id != binding.usf_content_profile_id {
            panic!(
                "USF execution plan route mismatch at scale {}: binding profile='{}', route profile='{}'",
                scale.index_from_top(),
                binding.usf_content_profile_id,
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
        app.init_resource::<ScaleContentRegistry>()
            .init_resource::<UsfActiveContentProfile>()
            .init_resource::<UsfExecutionPlan>()
            .add_systems(
                Startup,
                (
                    validate_scale_content_registry_system,
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

    fn scripted_scale_content_registry() -> ScaleContentRegistry {
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
                    usf_content_profile_id: "content_profile.test.default".to_string(),
                },
            );
        }

        ScaleContentRegistry {
            bindings_by_scale,
            known_dpt_samplers: HashSet::from([sampler_id]),
            known_dpt_categorizers: HashSet::from([categorizer_id]),
        }
    }

    #[test]
    fn scale_content_registry_scripted_definition_is_valid() {
        let registry = scripted_scale_content_registry();
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn scale_content_registry_rejects_missing_scale_binding() {
        let mut registry = scripted_scale_content_registry();
        registry.bindings_by_scale.remove(&Scale::MAX);
        let error = registry.validate().unwrap_err();
        assert!(error.contains("missing scale content binding"));
    }

    #[test]
    fn scale_content_registry_rejects_unknown_sampler_id() {
        let mut registry = scripted_scale_content_registry();
        let binding = registry.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.dpt_sampler_id = "dpt_sampler.unknown".to_string();
        let error = registry.validate().unwrap_err();
        assert!(error.contains("unknown dpt_sampler_id"));
    }

    #[test]
    fn scale_content_registry_rejects_empty_usf_content_profile_id() {
        let mut registry = scripted_scale_content_registry();
        let binding = registry.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.usf_content_profile_id = "".to_string();
        let error = registry.validate().unwrap_err();
        assert!(error.contains("empty usf_content_profile_id"));
    }

    fn scripted_active_content_profile() -> UsfActiveContentProfile {
        let mut known_zone_types = HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));

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
