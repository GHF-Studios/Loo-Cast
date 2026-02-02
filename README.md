# Loo Cast

**Loo Cast** is a long-term simulation and game engine built in Rust + Bevy, designed to render and simulate a fully scalable universe, from cosmic to subatomic, using a compositional architecture centered around _Scales_, _Phenomena_, and _Modularity_.

> "Every scale is the universe. Every phenomenon has its place. Mods fill the space between."

---

## 🧭 Project Structure

Loo Cast is both:
- **An Engine** — A runtime capable of simulating a scale-aware, contextual world using ECS + modular runtime logic.
- **A Game** — A mod layered on top of the engine that explores narrative and simulation possibilities within the universe model.

The two are linked statically and dynamically via Rust crates and modular build scripts.

---

## 🧱 Core Concepts

### Phenomena
- Abstract representations of "something that exists" in the game world.
- Each phenomenon is instantiated as one or more **Phenomena Models**, specific to a **Scale**.

### Scales
- Represent discrete layers of reality, each an entire 2D world (with faux-3D parallax).
- The world is simulated as an infinite stack of scales, each 10x larger than the one below.
- Movement between scales spawns/despawns detail.

### Phenomena Models
- ECS entities configured via a fixed archetype of components.
- Simulate local or aerial behavior via "internal" and "external" events.

### DPMs (Data Point Matrices)
- Probabilistic fields of generative information.
- Represent gradients of possibility across space and scale.

### ZLMs (Zone Lookup Maps)
- Map metric combinations from DPMs to biome-like zone types.
- Provide high-level meaning to otherwise statistical space.

---

## 🧩 Modularity

Loo Cast supports a **modding framework**:
- Core engine + core mod provide APIs, workflows, and ECS systems.
- Mods (like `base_mod`) are compiled separately and dynamically loaded.
- Mod code can implement functionality using the exposed plugin APIs and statics.
- Mod boundaries are enforced structurally and semantically.

---

## 🧠 Architecture Summary

- All ECS logic is driven through workflow systems and plugins.
- Builds produce an engine binary and mod DLLs/SOs, which are combined at runtime.

---

## 🛠️ Building & Running

Loo Cast uses a custom build system:
- Windows: `./build.ps1 [dev|fastdev|release]`
- Linux: `./build.sh [dev|fastdev|release]`

Then:
- `./run.ps1 [profile]` or `./run.sh [profile]`

Mod crates and assets are bundled into the `build/<profile>` folder.

---

## 🧬 Crate Breakdown

| Crate                | Role                                 |
|----------------------|--------------------------------------|
| `core_engine`        | Main binary. Links everything.       |
| `core_mod_api`       | Shared API for engine and mods       |
| `core_mod`           | Core systems and simulation logic    |
| `base_mod`           | Default mod implementing gameplay    |
| `*_macros`           | Proc macros for workflows and APIs   |
| `bevy_consumable_message` | Bevy plugin for consumable events |

---

## 🌌 Vision

Loo Cast aspires to simulate an *infinite world across infinite scales*, with seamless transitions, modding capabilities, and emergent simulation behavior. The engine is designed to support:

- Scale-aware procedural generation
- Dynamic level-of-detail via hierarchical context
- Cross-scale interaction propagation
- Game-as-zoom-lens design

---

## 📁 License

MIT (or your actual license here)
