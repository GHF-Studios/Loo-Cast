use crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH;
use crate::rhai_binding::bridges::testing::is_testing_top_level_module;

pub trait EngineExt {
    fn register_binding_graph(&mut self) -> &mut Self;
    fn register_binding_graph_with_testing(&mut self, include_testing_bridges: bool) -> &mut Self;
}

impl EngineExt for rhai::Engine {
    fn register_binding_graph(&mut self) -> &mut Self {
        self.register_binding_graph_with_testing(true)
    }

    fn register_binding_graph_with_testing(&mut self, include_testing_bridges: bool) -> &mut Self {
        for (top_level_module_path, top_level_module) in RUNTIME_BINDING_GRAPH().top_level_modules.iter() {
            let module_name = top_level_module_path.module_name();
            if !include_testing_bridges && is_testing_top_level_module(module_name.as_str()) {
                continue;
            }
            top_level_module.register_top_level_module(self);
        }
        self
    }
}
