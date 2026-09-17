# USF View / Zoom / LOD — Next Work Handoff

## Immediate direction

Do **not** spend time polishing the current S0 clay/lighting path. It is temporary realization plumbing.

### Pass B/C — first real spatial zoom
Implement together:

- `UsfViewFrame` / equivalent:
  - canonical observer anchor
  - current `SpatialScale`
  - fractional zoom toward an adjacent scale
- Scale-relative projection: each active scale is projected into bounded ordinary Bevy coordinates using units native to that scale. Never collapse giant canonical positions into global `f32`/`f64`.
- Simultaneously realize two adjacent scales during transitions.
- First acceptance test:
  - start at S0
  - zoom continuously toward S+1
  - S0 representation recedes while S+1 becomes dominant
  - canonical observer position remains identical
  - zoom back in cleanly

### Pass D — general multi-scale representation stack
Generalize the two-scale proof into a sparse active scale window around the observer.

Semantic state and representation remain separate:

```text
semantic scope/state
      +
view demand / active scale window
      ↓
scale-specific realization
      ↓
bounded meshes / physics / audio / etc.
```

Do **not** interpret this as 36 fully detailed meshes permanently loaded. It is a sparse nested representation stack.

### Pass E — make milestone scales visibly exist
Get ugly-but-real representations working quickly at approximately:

```text
S0   local terrain
S+1  landscape
S+2  regional terrain / biome structure
S+4  planet
S+10 planetary system
S+18 galaxy
S+22 cosmic-web region
S+35 root-scale universe
```

Intermediate scales still exist semantically and later gain their own realizations.

## Render distance / LOD requirement

Current local voxel render distance is far too small for useful testing.

Near-term:
- increase the local S0 streaming/materialization range substantially;
- keep generation/streaming demand-driven and budgeted so the range is not tied to one tiny hardcoded radius;
- use coarser representations farther from the observer instead of simply stopping visibility.

Long-term principle:

> USF/Loo Cast should trend toward visibility limited by physical information, occlusion, atmospheric/optical limits, available detail, and compute budgets — not by an arbitrary game-style render-distance wall.

This means the entire rendering problem is effectively one hierarchical, multi-domain LOD system:

```text
fine local realization
    ↓ distance / scale
coarser local realization
    ↓
regional realization
    ↓
planetary realization
    ↓
system / galactic / cosmological realization
```

Scale-spanning objects such as a planetary surface require their own hierarchical LOD/clipmap-like realization across many observer distances/scales.

The important architectural unification is:

- spatial scale transition,
- local render distance,
- distant-object LOD,
- semantic refinement,
- realization/materialization demand,

are distinct concerns, but must cooperate as one continuous visibility/representation system.

## After the scale-view stack exists

Then return to:
1. true volumetric S0 geology (mountains, valleys, caves, overhangs, strata),
2. biome/ecology realization,
3. material semantic state → procedural assets,
4. better lighting/presentation.

The current +35→0 semantic worldgen spine is the source state; the next work is making that hierarchy *viewable*.
