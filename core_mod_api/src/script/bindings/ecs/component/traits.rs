use rhai::Dynamic;

use super::types::Component;

pub trait ComponentCtor: Send + Sync + 'static {
    fn create(args: Dynamic) -> Component;
}
