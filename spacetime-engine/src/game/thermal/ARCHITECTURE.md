# Thermal / combustion vertical slice

This module is the first deliberately systemic environmental phenomenon in the
playground. The goal is not material-science fidelity yet; the goal is to keep
causal ownership clean enough that a more detailed USF model can replace each
approximation independently.

## Causal model

- `ThermalImpulse` transfers energy. Tools and other systems inject energy, not
  an `OnFire` flag.
- `ThermalBody` owns lumped temperature and heat capacity and exposes read-only diagnostic accessors.
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

Thermal/material state lives once on the semantic `UsfEntity`. Spatial heat
transfer uses only manifestations explicitly marked `ThermalSpatialSample`;
presentation may inspect every manifestation. A split or
multi-manifestation entity therefore keeps one thermal state while being able to
interact or appear in more than one spatial location.

The current propagation pass is O(n^2) over thermal semantic entities and uses
the closest manifestation pair. That is intentionally replaceable. World chunks,
USF coherence radii, spatial indexes, unloaded shadow state, or scale-specific
thermal approximations can change *candidate discovery* later without changing
the thermal domain contract.

## Explicit non-goals of this slice

- gas chemistry, oxygen concentration, smoke, convection, radiation, phase
  changes, detailed material decomposition, or spatial temperature fields;
- physically measured tuning of the current wood/human-like presets;
- large-world spatial acceleration;
- a universal "flammable means takes Health damage" abstraction.

Those belong in later mechanisms layered on the same state/event boundaries.


## Debug observability

Thermal state intentionally exposes read-only values such as temperature, fuel
energy and combustion power. A future generic developer-inspection subsystem can
register those values once and present them as compact inspector text, 3D
billboard annotations, vectors/volumes, or heat-map overlays without the thermal
simulation depending on any particular debug UI.
