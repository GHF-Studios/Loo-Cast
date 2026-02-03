# Bevy — Viewport & Host 🎯

**Purpose:** Describe how Bevy is used in this project as the renderer/viewport and as the application host.

- **Viewport / Rendering**
  - Bevy provides the rendering stack used for the viewport, camera, UI, and shader pipeline.
  - `core_engine` configures Bevy's `DefaultPlugins` and adjusts `WindowPlugin`, `ImagePlugin` (nearest sampling), and render-related defaults for the runtime.
  - Common rendering helpers (gizmos, color grading, lights) are registered and exposed to mods when necessary.

- **Host / App loop**
  - Bevy is the app host: it owns the schedule, event loop, and timing. `core_engine` composes Bevy plugins, registers `CoreApiPluginGroup`, and runs `App::run()`.
  - Diagnostics, UI (egui), and picking systems are configured via plugins.

- **Integration points**
  - Plugin authors should add `Plugin`s or `PluginGroup`s and register systems to the expected schedule stages; be conservative with global ordering and use labels/sets when possible.
  - Scripts and schedule hooks are installed as systems (see the schedule hook runner in `core_mod_api`).

- **Where to look in the code**
  - `core_engine/src/main.rs` — Bevy plugin configuration and app composition.
  - `core_mod_api/src/reflection/internals/functions.rs` — schedule-hook wiring and script runner code.
  - `core_mod/assets/scripts/` and `base_mod/assets/scripts/` — scripts that hook into Bevy's schedule stages.

💡 Tip: Keep rendering and UI work inside their own systems and rely on the engine's plugin groups for shared ordering and resources.
