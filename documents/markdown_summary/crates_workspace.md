# Workspace Crates

Current implementation responsibility split.

| Crate | Primary focus | Notes |
|---|---|---|
| `core_engine` | Binary/runtime entrypoint | Composes Bevy app + plugin groups and runs app loop. |
| `core_mod` | Engine init + assets | Engine-owned assets/config/scripts. |
| `core_mod_api` | Engine/runtime API | Internal code platform (workflows, USF runtime, Rhai runtime bindings). |
| `core_engine_macros` | Procedural macros | Active macro helpers for engine/runtime code. |
| `core_mod_macros` | Procedural macros | Integral crate, currently unused in this organization phase; active macro implementations are in `core_engine_macros`. |
| `base_mod` | Gameplay init + assets | Gameplay-facing scripts/configs/assets. |
| `base_mod_api` | Gameplay script bindings | Rhai-facing wrappers for gameplay scripting. |
| `base_mod_macros` | Procedural macros | Macro helpers for gameplay binding ergonomics. |
| `bevy_consumable_message` | Utility crate | Reusable Bevy utility helpers. |

## Related

- Runtime composition: `../intention_records/platform_records/10_runtime_composition.puml`
- Build packaging: `build_and_run.md`
