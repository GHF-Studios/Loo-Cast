use rhai::Dynamic;

use super::super::bindings::types::Component;

pub trait ComponentCtor: Send + Sync + 'static {
    fn create(args: Dynamic) -> Component;
}
