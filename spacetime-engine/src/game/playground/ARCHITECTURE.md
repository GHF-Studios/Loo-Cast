# Playground architecture

The playground is a reusable test surface, not a special-case game mode. Its
extension points are ordinary Bevy plugins, systems, resources, components and
messages so built-in content and Vapor-provided content use the same mechanisms.

## Direction of dependencies

```text
local devices / UI
        |
        v
playground input adapter
        |
        | UsePlaygroundItem { item, action, actor, aim }
        v
item plugin / action semantics
        |
        | domain-specific messages
        v
simulation domain (portal, combat, ECS, ...)
        |
        v
presentation
```

Dependencies should point downward through this diagram. In particular:

- Item plugins do not read keyboard/mouse state or cursor-capture state.
- The Portal subsystem does not know about the hotbar or Portal Gun.
- Local input does not mutate portal entities or execute item mechanics.
- Presentation state is derived from gameplay state instead of being mutated by
  unrelated gameplay systems.
- Stable logical IDs (`PlaygroundItemId`, `PlaygroundItemAction`) are the seam
  where static Vapor content can register additional behavior.

## Items and actions

`PlaygroundCatalog` contains item metadata only. Behavior lives in ordinary
plugins which consume `UsePlaygroundItem` messages addressed to their logical
item ID.

The built-in input adapter maps:

- left mouse -> `PlaygroundItemAction::PRIMARY`;
- right mouse -> `PlaygroundItemAction::SECONDARY`;
- `R` -> `PlaygroundItemAction::RELOAD`;
- middle mouse -> the playground-global erase tool.

`PlaygroundItemAction` is an extensible string ID rather than a closed enum.
Mods can define additional logical actions and produce the same messages from
other devices, AI, replay or network input.

`PlaygroundAim` is a shared per-frame actor + aim snapshot. Presentation helpers
such as laser sights can consume it without coupling themselves to a particular
player camera implementation.

## Portal ownership

The portal pair owns stable entities because recursive rendering keeps entity
references and render infrastructure associated with them. Gameplay therefore
places/removes portals with `PortalCommand` rather than despawning or directly
mutating those entities.

A removed endpoint is logically inactive while its entity remains alive. The
pair becomes visible and traversable only when both endpoints are active. This
lets the Portal Gun replace either endpoint cheaply while preserving the render
tree.

Authored map portal markers are bootstrap data only. Once they initialize the
pair, the map no longer overwrites runtime portal placement.

## Player and camera boundaries

The player body owns simulation-facing state (`Transform`, physical collider,
stance, aim intent, movement intent). Camera placement is presentation.

Third-person camera zoom deliberately separates:

1. authored base boom distance;
2. persistent user zoom offset;
3. transient collision-constrained resolved distance.

Collision never writes back into the desired boom distance, so walking behind a
wall can push the camera inward without destroying the player's previous zoom.

## Modding

Static Vapor mods can add a plugin which registers catalog entries and systems
using the same public resources/messages as built-in content. A new item usually
needs only:

1. a stable `PlaygroundItemId`;
2. a startup system registering its metadata;
3. one or more systems consuming `UsePlaygroundItem` for that ID;
4. domain-specific messages/components for the actual mechanic.

Runtime-loaded scripting/registries may be layered on later, but built-in code
should not depend on that future mechanism to remain decoupled today.

## Entity splitting prototype

The first portal-splitting experiment deliberately uses reusable USF/ECS
primitives rather than representing a split as a special Portal Gun clone.

A semantic entity is an entity carrying `UsfEntity`. Concrete spatial bodies
relate to it through `UsfManifestationOf`, and the semantic root exposes all of
them through `UsfManifestations`. `UsfManifestationAuthority` marks the one
manifestation that legacy systems may continue to treat as authoritative while
those systems are not yet manifestation-aware.

The intended split-aware rule is:

- identity, inventory, health and other singular semantics normally execute
  once on the semantic entity;
- transform, collision, rendering and other spatial concerns execute per
  manifestation;
- a system that cannot support multiple manifestations yet should say so by
  consuming the authoritative manifestation rather than silently assuming that
  every semantic entity has exactly one body.

`physics::topology` contains mechanism-independent helpers for the prototype:
`SpatialSplitBox` can be partitioned by an arbitrary rigid world plane, and
`KinematicQueryExclusions` lets topology temporarily remove specific colliders
from manual movement queries. The portal module supplies only the portal plane,
rigid mapping and host-surface exclusions.

Current prototype limitation: one authoritative manifestation uses one reserve
peer for one active spatial split at a time. This is intentionally narrower than
the eventual generalized manifestation runtime, but the semantic relationship,
partition geometry and collision-query seam are reusable outside portals.

The current portal adapter is intentionally a pressure-test rather than the
final split runtime. In particular:

- only the player authority is opted in today; arbitrary entities can reuse the
  manifestation/topology primitives but do not split automatically;
- the reserve peer has a real clipped Avian collider but no split-aware visual
  mesh yet;
- the authoritative character motor is still singular, so destination-side
  contacts before center crossing do not yet feed a coupled constraint back
  into source-side locomotion;
- crossing time is reconstructed from the motor tick's start/end segment rather
  than from every internal slide segment;
- one fixed tick resolves at most one center crossing, and crouch clearance is
  still validated from the authoritative manifestation.

Those are expected integration seams, not contracts of the generic
manifestation relationship or box/plane partitioner.

A different splitting mechanic does not need the portal module. Its minimum
prototype path is: create/identify peer `UsfManifestationOf` entities, choose an
authoritative manifestation, use `partition_box_by_plane` (or a future shape
partitioner) to derive manifestation-local collision geometry, and write
`KinematicQueryExclusions` for any topology-local holes its kinematic queries
must observe. The mapping between manifestations is mechanism-owned; portals
currently use their rigid source-to-destination transform for that step.
