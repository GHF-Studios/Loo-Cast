pub mod shop;

pub const TESTING_TOP_LEVEL_MODULES: &[&str] = &["shop"];

pub fn is_testing_top_level_module(module_name: &str) -> bool {
    TESTING_TOP_LEVEL_MODULES.contains(&module_name)
}
