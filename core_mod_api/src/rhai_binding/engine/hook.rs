use crate::bevy::prelude::{Mut, World as BevyWorld};
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};
use crate::script::ecs::world::bindings::types::World;
use std::path::{Path, PathBuf};

fn collect_rhai_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read companion hook dir '{}': {e}", dir.display()));

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if path.is_dir() {
            collect_rhai_files_recursive(&path, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rhai") {
            out.push(path);
        }
    }
}

fn compose_hook_source(path: &str) -> String {
    let script_path = PathBuf::from(path);
    let companion_dir = script_path.with_extension("");

    let mut source_parts: Vec<String> = Vec::new();

    if companion_dir.is_dir() {
        let mut companion_files = Vec::new();
        collect_rhai_files_recursive(&companion_dir, &mut companion_files);
        companion_files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

        for file in companion_files {
            let code = std::fs::read_to_string(&file)
                .unwrap_or_else(|e| panic!("Failed to read companion hook file '{}': {e}", file.display()));
            source_parts.push(code);
        }
    }

    let main_hook_code = std::fs::read_to_string(&script_path)
        .unwrap_or_else(|e| panic!("Failed to read hook file '{}': {e}", script_path.display()));
    source_parts.push(main_hook_code);

    source_parts.join("\n\n")
}

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            let hook_code = compose_hook_source(&path);
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();

            let world = std::mem::take(source_world);
            let world_raw_handle: AccessCell<Scoped, BevyWorld> = AccessCell::new(world);
            let world_binding = World {
                world: world_raw_handle.clone(),
            };
            let shared_world = rhai::Shared::new(world_binding);

            engine
                .call_fn::<()>(&mut scope, &ast, "main", (shared_world,))
                .unwrap();

            let returned_world = world_raw_handle.take();
            *source_world = returned_world;
        });
    }
}
