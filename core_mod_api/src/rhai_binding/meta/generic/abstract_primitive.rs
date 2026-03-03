
pub const trait ConstDynMetadata: 'static + Clone + Send + Sync {
    fn raw_rust_module_path(&self) -> &'static str;
}