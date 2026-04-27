# core_mod — Engine code & assets

`core_mod` is the authoritative crate for engine-level assets and built-in data of any kind.
- It bundles non-gameplay-related/engine-related configuration files, scripts, models, shaders, textures, and other content used by the engine at runtime.
- It also bundles the rust code found in the `core_mod_api` crate.
- The crate exposes an `api_initializer!` hook used by the engine to register global statics and connect assets to the `core_mod_api` code surface.
- Keep non-gameplay assets here, and gameplay-related assets in `base_mod`.

See also: `documents/markdown_summary/assets_and_ownership.md` for canonical paths and conventions.
