use crate::rhai_binding::meta::registry::RuntimeBindingGraph;
use core_mod_macros::export_static;
use once_cell::sync::Lazy;

export_static!(self, crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH: Lazy<RuntimeBindingGraph> = Lazy::new(|| RuntimeBindingGraph::build()));
