//! Path registration and lookup for typed runtime variables.

use super::*;

#[derive(Resource, Default)]
pub struct RuntimeVariableRegistry {
    bindings: BTreeMap<String, RuntimeVariableBinding>,
}

impl RuntimeVariableRegistry {
    pub fn register(&mut self, binding: RuntimeVariableBinding) {
        let path = normalize_path(binding.spec().path);
        assert!(!path.is_empty(), "runtime-variable paths must not be empty");
        assert!(
            !self.bindings.contains_key(&path),
            "duplicate runtime-variable path `{path}`"
        );
        self.bindings.insert(path, binding);
    }

    pub(crate) fn binding(&self, path: &str) -> Option<RuntimeVariableBinding> {
        self.bindings.get(&normalize_path(path)).copied()
    }

    pub(crate) fn path_completions(&self, prefix: &str) -> Vec<String> {
        let prefix = normalize_path(prefix);
        self.bindings
            .keys()
            .filter(|path| path.starts_with(&prefix))
            .cloned()
            .collect()
    }

    /// Registry owns path indexing; command presentation only sees bindings.
    pub(super) fn matching<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = RuntimeVariableBinding> + 'a {
        self.bindings
            .iter()
            .filter(move |(path, _)| path.starts_with(prefix))
            .map(|(_, binding)| *binding)
    }

    pub(crate) fn value_completions(&self, path: &str, prefix: &str) -> Vec<String> {
        self.binding(path).map_or_else(Vec::new, |binding| {
            binding.spec().domain.completions(prefix)
        })
    }
}

pub trait AppRuntimeVariableExt {
    fn register_runtime_variable(&mut self, binding: RuntimeVariableBinding) -> &mut Self;
}

impl AppRuntimeVariableExt for App {
    fn register_runtime_variable(&mut self, binding: RuntimeVariableBinding) -> &mut Self {
        self.init_resource::<RuntimeVariableRegistry>();
        self.world_mut()
            .resource_mut::<RuntimeVariableRegistry>()
            .register(binding);
        self
    }
}

pub(super) fn normalize_path(path: &str) -> String {
    path.trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}
