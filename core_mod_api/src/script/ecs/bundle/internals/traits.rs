use rhai::Dynamic;

use super::super::bindings::types::Bundle;

pub trait BundleFromDynamic: Send + Sync + 'static {
    /// Construct a Bundle from a method name and Dynamic parameters.
    /// Allows flexible bundle construction with different constructor variants.
    /// 
    /// # Arguments
    /// * `method` - Constructor method name (e.g., "default", "with_position", "with_zoom")
    /// * `params` - Dynamic parameters (typically a Rhai Map) passed to the constructor
    fn from_dynamic(method: &str, params: Dynamic) -> Bundle;
}
