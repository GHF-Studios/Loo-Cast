use crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH;

pub trait EngineExt {
    fn register_binding_graph(&mut self) -> &mut Self;
}

impl EngineExt for rhai::Engine {
    fn register_binding_graph(&mut self) -> &mut Self {
        for (_top_level_module_path, top_level_module) in RUNTIME_BINDING_GRAPH().top_level_modules.iter() {
            top_level_module.register_top_level_module(self);
        }
        self
    }
}
