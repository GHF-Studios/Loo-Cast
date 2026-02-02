# Loo Cast

// Cut out the entire header, cause it was generic and said a lot but also nothing

## 🧭 Project Structure

Loo Cast is both:
- **An Engine** — A runtime capable of simulating a scale-aware, contextual world using ECS + modular runtime logic.
- **A Game** — A mod layered on top of the engine that explores narrative and simulation possibilities within the universe model.

The two are linked statically and dynamically via Rust crates and modular build scripts.

---

## 🧱 Core Concepts // This is lackluster at best, a lot of sections are very incomplete or straight up missing. Either you fully explain the USF, or you do that at a later point, but not half/half, yk?

### Phenomena
- Abstract representations of "something that exists" in the game world.
- Each phenomenon is instantiated as one or more **Phenomena Models**, each specific to a **Scale**.

### Scales // This was missing the juicy details, plus I made a live realization regarding the design, lol
- Represent discrete layers of reality, each an entire 3D world, but only a slice can be viewed, and there is one slice for each scale, stacking on top of each other. Stuff that is in the background, so large planets, moons, galaxies, as seen form, say a huamn perspective, are not only larger, but physically further away in the z- direction, LOGARITHMICALLY SO!!! So the stuff we see in the background that is 100x larger is also 100x as far away from the camera... to make the visuals possible, the game has to be 3D. Yeah but, like, each scale is it's own sort of level of detail, but the weird thing is that the stuff that is in the lower scales, does not exist for the higher-scale structrures we see in the background, because they ONLY exist at that high level, but ofc when move around near them, the game will simulate more and more detail, and we see the thing visually moving towards the center of our view as we move towards it, and as we zoom in on it, it's detail get's spawned and manifests at our scale. // Changed this a lot, but it still needs to be cleaned up/structured a bit better I think
- The world is simulated as an infinite stack of scales, each 10x larger than the one below.
- Movement between scales spawns/despawns detail.

### Phenomena Models // Lacking certain details/pointers to other related concepts
- ECS entities configured via a fixed archetype of components. // Very good
- Simulate local or aerial behavior via "internal" and "external" events. // Way too simplified; events are a complex backbone of the USF

### DPMs (Data Point Matrices) // Very good, but missing a lot of info
- Probabilistic fields of generative information.
- Represent gradients of possibility across space and scale.

### ZLMs (Zone Lookup Maps) // Did some small changes + Should we call it map metrics? Sounds reasonable actually, mroe than just "metrics"
- Map metric combinations from DPMs to biome-like zone types.
- Provide high-level meaning to otherwise statistical space.
- Each scale has a single dedicated ZLM for the scale-local map metrics

### Map Metrics // Fully missing

### Events // Like 99% missing

### State (of Phenomena Models) // Fully missing

### Context & Scale-based stack-like structure (of Phenomenas and Phenomena Models Models, and also Chunks which house the aforementioned) // Fully missing

### Constraints, Templates, and Constraint-Template-Sets // Fully missing

### Dynamic Generation & Updates // Fully missing as a summary section; World gen and world updating (and world destruction) is never really discussed directly

### Phenomena Model Evolution // Fully missing

### Meta Changes // Fully missing

---

## 🧩 Modularity

Loo Cast supports a **modding framework**:
- Core engine + core mod provide APIs, workflows, and ECS systems.
- Mods (like `base_mod`) are compiled separately and dynamically loaded.
- Mod code can implement functionality using the provided APIs and orchestrate it using the script bindings provided by the APIs.

---

## 🧠 Architecture Summary

- All ECS logic is driven through workflow systems and plugins. // Idk, that's just not true. workflows are a tool for complex tasks involving the gpu mostly, and plugins are a way to configure bevy, but there is no real proper working modding capabilities really yet.
- Builds produce an engine binary and mod DLLs/SOs, which are combined at runtime. // Yes, simple, albeit true.

---

## 🛠️ Building & Running // Actually just good as it is mostly

Loo Cast uses a custom build system:
- Windows: `./build.ps1 [dev|fastdev|release]`
- Linux: `./build.sh [dev|fastdev|release]`

Then:
- `./run.ps1 [profile]` or `./run.sh [profile]`

Mod crates and assets are bundled into the `build/<profile>` folder.

---

## 🧬 Crate Breakdown

| Crate                      | Role                                         |
|----------------------------|----------------------------------------------|
| `core_engine`              | Main binary. Links everything.               |
| `core_mod`                 | Bundles core_mod_api and core assets         |
| `core_mod_api`             | Core API for engine and mods                 |
| `core_mod_macros`          | Shared macros for engine and mods            |
| `base_mod`                 | Bundles core_mod_api and gameplay assets     |
| `base_mod_api`             | Base API for base gameplay + mods            |
| `base_mod_macros`          | Shared macros for base gameplay and mods     |
| `bevy_consumable_message`  | Forked bevy plugin for consumable messages   |
