# Thermal / combustion vertical slice

This module is the first deliberately systemic environmental phenomenon in the
playground. The goal is not material-science fidelity yet; the goal is to keep
causal ownership clean enough that a more detailed USF model can replace each
approximation independently.

## Causal model

- `ThermalImpulse` transfers energy. Tools and other systems inject energy, not
  an `OnFire` flag.
- `ThermalBody` owns aggregate temperature and heat capacity and remains the semantic thermal state consumed by existing gameplay systems.
- `ThermalMaterial` carries homogeneous solid density, specific heat and thermal conductivity in SI units.
- `ThermalField` is an optional local finite-volume refinement. Its cells store energy in joules, derive temperature from the material, and exchange equal-and-opposite energy through shared faces.
- `ThermalPointImpulse` preserves hit position so tools can heat or cool one local cell instead of erasing gradients immediately.
- `CombustibleMaterial` says how a material can burn.
- `Fuel` owns consumable chemical energy.
- `Combustion` is derived runtime state. Ignition occurs when temperature and
  fuel satisfy material conditions.
- combustion self-heats and transfers heat to nearby semantic entities using
  their manifestations as spatial samples.
- `ThermalInjury` is a biological adapter from temperature to generic `Damage`.
  Inanimate degradation should be a separate adapter rather than pretending all
  objects respond like living Health.
- flame meshes and lights are presentation derived from `Combustion`.

## Manifestations

Thermal/material state lives once on the semantic `UsfEntity`. `ThermalField`
is also semantic state: split manifestations do not each get a private copy of
the object's internal energy. A localized world-space impulse resolves through
the struck manifestation into that shared field's local coordinates.

Spatial heat transfer uses only manifestations explicitly marked
`ThermalSpatialSample`; presentation may inspect every manifestation. A split or
multi-manifestation entity therefore keeps one thermal state while being able to
interact or appear in more than one spatial location.

The current propagation pass is O(n^2) over thermal semantic entities and uses
the closest manifestation pair. That is intentionally replaceable. World chunks,
USF coherence radii, spatial indexes, unloaded shadow state, or scale-specific
thermal approximations can change *candidate discovery* later without changing
the thermal domain contract.

## Solid thermal refinement

The first spatial representation is deliberately a regular local box grid. A
fixed 60 Hz thermal clock advances isotropic Fourier conduction. The solver uses
cell face area, center distance and material conductivity, automatically
subdividing a step when required by the explicit diffusion stability bound.
Internal transfers are accumulated as equal-and-opposite joule deltas, so they
redistribute rather than create or destroy field energy.

`ThermalBody` currently remains the aggregate authority for compatibility with
existing combustion/cooling/injury systems. After those lumped mechanisms run,
any aggregate energy change is projected uniformly back into `ThermalField`.
New localized mechanisms should operate on the field directly so spatial
information is retained.

## Explicit non-goals of this slice

- gas chemistry, oxygen concentration, smoke, convection, radiation, phase
  changes, detailed material decomposition, or object-to-object contact conduction;
- temperature-dependent/anisotropic material properties or geometrically cut cells;
- unifying thermal properties with rigid-body mass or renderer materials;
- physically measured tuning of the current combustion/human-like presets;
- large-world spatial acceleration;
- a universal "flammable means takes Health damage" abstraction.

Those belong in later mechanisms layered on the same state/event boundaries.


## Debug observability

Thermal observability exposes aggregate temperature plus the internal field's
minimum/maximum temperature, SI material properties and per-cell temperature
samples. The generic observability layer renders those cells as a local-space
thermal point cloud on each active manifestation; the simulation itself remains
independent from debug rendering. The pre-existing combustion-coupling scalar
field remains available separately.
