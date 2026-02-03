# base_mod — Gameplay mod bundle

`base_mod` is the gameplay mod that ships with the engine. It bundles gameplay-related rust code, rhai scripts, assets(such as configs, scripts, shaders, etc.), and the initialization glue that exposes these assets to the runtime.

What lives here

- `base_mod/assets/scripts/` — Assets (configs, scripts, models, UI, audio, etc.) that are specific to gameplay features.

Notes

- Keep gameplay logic data-driven where possible: prefer scripts and configs over hard-coded behavior.
- Use `base_mod_api` to expose safe wrappers to scripts and to translate between runtime types and scripting types.

See also: `docs/Modding.md` and `docs/Assets.md`.