use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::config::{statics::CONFIG, types::ConfigValue};
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{
    USF_CONTENT_PACKAGES_BY_ID, USF_CONTENT_PROFILES_BY_ID, USF_DPT_CATEGORIZER_IDS, USF_DPT_SAMPLER_IDS, USF_PACKAGE_CONTRIBUTIONS_BY_ID,
    USF_SCALE_BINDINGS_BY_SCALE,
};
use crate::usf::scale::Scale;

pub const DEFAULT_USF_CONTENT_PROFILE_ID: &str = "content_profile.placeholder_gameplay.v1";
pub const PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID: &str = "content_package.placeholder_gameplay.v1";
pub const PLACEHOLDER_GAMEPLAY_CONFIG_ENABLED_KEY: &str = "usf_content/content_packages/placeholder_gameplay/enabled";

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
        let script_samplers = script_dpt_samplers();
        let script_categorizers = script_dpt_categorizers();
        let script_bindings = script_scale_bindings();
        let script_authored = !script_samplers.is_empty() || !script_categorizers.is_empty() || !script_bindings.is_empty();

        let registry = if script_authored {
            Self {
                bindings_by_scale: script_bindings,
                known_dpt_samplers: script_samplers,
                known_dpt_categorizers: script_categorizers,
            }
        } else {
            let mut bindings_by_scale = HashMap::new();
            for index in 0..Scale::SCALE_LEVEL_COUNT {
                let Some(scale) = Scale::from_index_from_top(index) else {
                    continue;
                };
                bindings_by_scale.insert(scale, baseline_scale_binding(scale));
            }
            Self {
                bindings_by_scale,
                known_dpt_samplers: default_dpt_samplers(),
                known_dpt_categorizers: default_dpt_categorizers(),
            }
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

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfContentPackageDefinition {
    pub default_enabled: bool,
    pub config_enabled_key: String,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfContentPackageRegistry {
    pub packages_by_id: HashMap<String, UsfContentPackageDefinition>,
}
impl Default for UsfContentPackageRegistry {
    fn default() -> Self {
        let script_packages = script_content_packages();
        let registry = if script_packages.is_empty() {
            Self {
                packages_by_id: default_content_packages(),
            }
        } else {
            Self {
                packages_by_id: script_packages,
            }
        };

        if let Err(reason) = registry.validate() {
            panic!("USF content package registry default validation failed: {reason}");
        }

        registry
    }
}
impl UsfContentPackageRegistry {
    pub fn package_definition(&self, content_package_id: &str) -> Option<&UsfContentPackageDefinition> {
        self.packages_by_id.get(content_package_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.packages_by_id.is_empty() {
            return Err("no USF content packages registered".to_string());
        }

        for (content_package_id, package) in &self.packages_by_id {
            if content_package_id.trim().is_empty() {
                return Err("USF content package id must not be empty".to_string());
            }
            if package.config_enabled_key.trim().is_empty() {
                return Err(format!("USF content package '{}' has an empty config_enabled_key", content_package_id));
            }
        }

        Ok(())
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfContentProfileDefinition {
    pub content_package_ids: Vec<String>,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfContentProfileRegistry {
    pub profiles_by_id: HashMap<String, UsfContentProfileDefinition>,
}
impl Default for UsfContentProfileRegistry {
    fn default() -> Self {
        let script_profiles = script_content_profiles();
        let registry = if script_profiles.is_empty() {
            Self {
                profiles_by_id: default_content_profiles(),
            }
        } else {
            Self {
                profiles_by_id: script_profiles,
            }
        };

        if let Err(reason) = registry.validate() {
            panic!("USF content profile registry default validation failed: {reason}");
        }

        registry
    }
}
impl UsfContentProfileRegistry {
    pub fn content_packages_for_profile(&self, profile_id: &str) -> Option<&[String]> {
        self.profiles_by_id.get(profile_id).map(|profile| profile.content_package_ids.as_slice())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profiles_by_id.is_empty() {
            return Err("no USF content profiles registered".to_string());
        }

        for (profile_id, profile) in &self.profiles_by_id {
            if profile_id.trim().is_empty() {
                return Err("USF content profile id must not be empty".to_string());
            }
            if profile.content_package_ids.is_empty() {
                return Err(format!("USF content profile '{}' must contain at least one content package", profile_id));
            }
            let mut seen = HashSet::<String>::new();
            for content_package_id in &profile.content_package_ids {
                if content_package_id.trim().is_empty() {
                    return Err(format!("USF content profile '{}' has an empty content_package_id entry", profile_id));
                }
                if !seen.insert(content_package_id.clone()) {
                    return Err(format!(
                        "USF content profile '{}' contains duplicate content package '{}'",
                        profile_id, content_package_id
                    ));
                }
            }
        }

        Ok(())
    }
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

#[derive(Resource, Reflect, Debug, Clone, Default)]
#[reflect(Resource)]
pub struct UsfContentPackageActivation {
    pub enabled_packages: HashSet<String>,
}
impl UsfContentPackageActivation {
    pub fn is_enabled(&self, content_package_id: &str) -> bool {
        self.enabled_packages.contains(content_package_id)
    }
}

fn default_dpt_samplers() -> HashSet<String> {
    HashSet::from(["dpt_sampler.debug.default.v1".to_string()])
}

fn default_dpt_categorizers() -> HashSet<String> {
    HashSet::from(["dpt_categorizer.debug.zlm_lookup.v1".to_string()])
}

fn baseline_scale_binding(_scale: Scale) -> ScaleContentBinding {
    ScaleContentBinding {
        dpt_sampler_id: "dpt_sampler.debug.default.v1".to_string(),
        dpt_categorizer_id: "dpt_categorizer.debug.zlm_lookup.v1".to_string(),
        chunk_store_key: "chunk_store.default".to_string(),
        usf_content_profile_id: DEFAULT_USF_CONTENT_PROFILE_ID.to_string(),
    }
}

fn script_dpt_samplers() -> HashSet<String> {
    USF_DPT_SAMPLER_IDS().lock().unwrap().clone()
}

fn script_dpt_categorizers() -> HashSet<String> {
    USF_DPT_CATEGORIZER_IDS().lock().unwrap().clone()
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

fn default_content_packages() -> HashMap<String, UsfContentPackageDefinition> {
    HashMap::from([(
        PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID.to_string(),
        UsfContentPackageDefinition {
            default_enabled: true,
            config_enabled_key: PLACEHOLDER_GAMEPLAY_CONFIG_ENABLED_KEY.to_string(),
        },
    )])
}

fn script_content_packages() -> HashMap<String, UsfContentPackageDefinition> {
    USF_CONTENT_PACKAGES_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(content_package_id, package)| {
            (
                content_package_id.clone(),
                UsfContentPackageDefinition {
                    default_enabled: package.default_enabled,
                    config_enabled_key: package.config_enabled_key.trim().to_ascii_lowercase(),
                },
            )
        })
        .collect()
}

fn default_content_profiles() -> HashMap<String, UsfContentProfileDefinition> {
    HashMap::from([(
        DEFAULT_USF_CONTENT_PROFILE_ID.to_string(),
        UsfContentProfileDefinition {
            content_package_ids: vec![PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID.to_string()],
        },
    )])
}

fn script_content_profiles() -> HashMap<String, UsfContentProfileDefinition> {
    USF_CONTENT_PROFILES_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(profile_id, profile)| {
            (
                profile_id.clone(),
                UsfContentProfileDefinition {
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

fn content_package_enabled_from_config(content_package_id: &str, package: &UsfContentPackageDefinition) -> bool {
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

fn validate_usf_content_package_registry_system(registry: Res<UsfContentPackageRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF content package registry validation failed: {reason}");
    }
}

fn validate_usf_content_profile_registry_system(registry: Res<UsfContentProfileRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF content profile registry validation failed: {reason}");
    }
}

fn rebuild_usf_execution_plan_system(
    mut execution_plan: ResMut<UsfExecutionPlan>,
    scale_content_registry: Res<ScaleContentRegistry>,
    content_profile_registry: Res<UsfContentProfileRegistry>,
    content_package_registry: Res<UsfContentPackageRegistry>,
) {
    execution_plan.routes_by_scale.clear();
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
        let content_package_ids = content_profile_registry
            .content_packages_for_profile(binding.usf_content_profile_id.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "USF execution plan rebuild missing profile '{}' for scale index {}",
                    binding.usf_content_profile_id,
                    scale.index_from_top()
                )
            });
        let mut normalized_content_package_ids = Vec::<String>::new();
        let mut seen = HashSet::<String>::new();
        for content_package_id in content_package_ids {
            if !seen.insert(content_package_id.clone()) {
                panic!(
                    "USF execution plan rebuild found duplicate content package '{}' in profile '{}' at scale index {}",
                    content_package_id,
                    binding.usf_content_profile_id,
                    scale.index_from_top()
                );
            }
            if content_package_registry.package_definition(content_package_id).is_none() {
                panic!(
                    "USF execution plan rebuild missing content package '{}' resolved from profile '{}' at scale index {}",
                    content_package_id,
                    binding.usf_content_profile_id,
                    scale.index_from_top()
                );
            }
            normalized_content_package_ids.push(content_package_id.clone());
        }

        execution_plan.routes_by_scale.insert(
            scale,
            UsfScaleExecutionRoute {
                dpt_sampler_id: binding.dpt_sampler_id.clone(),
                dpt_categorizer_id: binding.dpt_categorizer_id.clone(),
                chunk_store_key: binding.chunk_store_key.clone(),
                usf_content_profile_id: binding.usf_content_profile_id.clone(),
                content_package_ids: normalized_content_package_ids,
            },
        );
    }
}

fn validate_usf_execution_plan_system(execution_plan: Res<UsfExecutionPlan>, scale_content_registry: Res<ScaleContentRegistry>) {
    let selected_packages = USF_PACKAGE_CONTRIBUTIONS_BY_ID().lock().unwrap().keys().cloned().collect::<HashSet<_>>();

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
            if !selected_packages.is_empty() && !selected_packages.contains(content_package_id) {
                panic!(
                    "USF execution plan route references package '{}' at scale {} but it was not composed by bootstrap",
                    content_package_id,
                    scale.index_from_top()
                );
            }
        }
    }
}

fn rebuild_usf_content_package_activation_system(
    mut activation: ResMut<UsfContentPackageActivation>,
    execution_plan: Res<UsfExecutionPlan>,
    content_package_registry: Res<UsfContentPackageRegistry>,
) {
    activation.enabled_packages.clear();
    let mut seen_package_ids = HashSet::<String>::new();
    for route in execution_plan.routes_by_scale.values() {
        for content_package_id in &route.content_package_ids {
            if !seen_package_ids.insert(content_package_id.clone()) {
                continue;
            }

            let package = content_package_registry
                .package_definition(content_package_id.as_str())
                .unwrap_or_else(|| panic!("USF content activation rebuild missing package '{}'", content_package_id));
            if content_package_enabled_from_config(content_package_id.as_str(), package) {
                activation.enabled_packages.insert(content_package_id.clone());
            }
        }
    }
}

pub(crate) struct ContentPlugin;
impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScaleContentRegistry>()
            .init_resource::<UsfContentPackageRegistry>()
            .init_resource::<UsfContentProfileRegistry>()
            .init_resource::<UsfExecutionPlan>()
            .init_resource::<UsfContentPackageActivation>()
            .add_systems(
                Startup,
                (
                    validate_scale_content_registry_system,
                    validate_usf_content_package_registry_system,
                    validate_usf_content_profile_registry_system,
                    rebuild_usf_execution_plan_system,
                    validate_usf_execution_plan_system,
                    rebuild_usf_content_package_activation_system,
                )
                    .chain()
                    .in_set(AppSet::Diagnostics),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_scale_content_registry() -> ScaleContentRegistry {
        let mut bindings_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            bindings_by_scale.insert(scale, baseline_scale_binding(scale));
        }

        ScaleContentRegistry {
            bindings_by_scale,
            known_dpt_samplers: default_dpt_samplers(),
            known_dpt_categorizers: default_dpt_categorizers(),
        }
    }

    #[test]
    fn scale_content_registry_baseline_is_valid() {
        let registry = baseline_scale_content_registry();
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn scale_content_registry_rejects_missing_scale_binding() {
        let mut registry = baseline_scale_content_registry();
        registry.bindings_by_scale.remove(&Scale::MAX);
        let error = registry.validate().unwrap_err();
        assert!(error.contains("missing scale content binding"));
    }

    #[test]
    fn scale_content_registry_rejects_unknown_sampler_id() {
        let mut registry = baseline_scale_content_registry();
        let binding = registry.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.dpt_sampler_id = "dpt_sampler.unknown".to_string();
        let error = registry.validate().unwrap_err();
        assert!(error.contains("unknown dpt_sampler_id"));
    }

    #[test]
    fn scale_content_registry_rejects_empty_usf_content_profile_id() {
        let mut registry = baseline_scale_content_registry();
        let binding = registry.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.usf_content_profile_id = "".to_string();
        let error = registry.validate().unwrap_err();
        assert!(error.contains("empty usf_content_profile_id"));
    }

    #[test]
    fn content_profile_registry_rejects_empty_package_list() {
        let registry = UsfContentProfileRegistry {
            profiles_by_id: HashMap::from([(
                "content_profile.test.empty_packages".to_string(),
                UsfContentProfileDefinition {
                    content_package_ids: Vec::new(),
                },
            )]),
        };
        let error = registry.validate().unwrap_err();
        assert!(error.contains("must contain at least one content package"));
    }

    #[test]
    fn content_profile_registry_rejects_duplicate_package_ids() {
        let registry = UsfContentProfileRegistry {
            profiles_by_id: HashMap::from([(
                "content_profile.test.duplicate_packages".to_string(),
                UsfContentProfileDefinition {
                    content_package_ids: vec!["content_package.a".to_string(), "content_package.a".to_string()],
                },
            )]),
        };
        let error = registry.validate().unwrap_err();
        assert!(error.contains("contains duplicate content package"));
    }
}
