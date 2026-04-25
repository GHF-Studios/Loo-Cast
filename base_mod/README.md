# base_mod — Gameplay mod bundle

`base_mod` is the gameplay mod that ships with the engine. It bundles gameplay-related Rust code, assets (configs, content declarations, shaders, etc.), and initialization glue that exposes those assets to runtime systems.

What lives here

- `base_mod/assets/*CONTENT_DOMAIN*/` — gameplay content-domain roots (for example: `modpack`, `mod`, `scale`, `metric`, `phenomenon_realizer`, `phenomenon`, `texture`, `sfx`, `music`, `shader`, `config`, `usf_texture`, `usf_sfx`, `usf_model`).
- `base_mod/assets/*MY_ASSET_TYPE*/` — gameplay asset families as needed by the mod.

Notes

- Keep gameplay logic data-driven where possible: prefer scripts and configs over hard-coded behavior.
- Use `base_mod_api` to expose capability wrappers and to translate between runtime types and scripting-facing types.

See also: `documents/markdown_summary/modding_status.md` and `documents/markdown_summary/assets_and_ownership.md`.
