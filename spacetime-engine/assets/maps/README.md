# Authored maps

`.spacemap` files are hot-reloadable RON assets interpreted by `geometry::AuthoredGeometryPlugin`.
They describe runtime geometry and generic anchors; gameplay-specific meaning stays in adapter systems.

## Coordinates and units

All authored distances are metres. Speeds and accelerations do not currently appear in the geometry
format; where physical quantities are added, SI is the default. Angles remain expressed in degrees.

Transforms use `(x, y, z)` metres and Euler `(x, y, z)` degrees. Y is up. For directed generators, yaw `0`
points along local/world `+Z`, `90` along `+X`, `-90` along `-X`.

## Primitive objects

- `Box`: oriented cuboid; may be visual-only with `solid: false`.
- `Ramp`: rectangular inclined slab. `start` is the centre of the lower edge of its **top surface**;
  `run` and `rise` describe the top surface exactly.
- `Cylinder`, `Sphere`, `Capsule`: round collision/visual primitives.
- `ConvexPrism`: convex XY polygon extruded along local Z.
- `MovingBox`: kinematic cuboid oscillating smoothly between its base and `base + travel`.
- `Marker`: non-rendering named transform with arbitrary `kind` and tags. Game code decides whether a
  marker means spawn, portal anchor, checkpoint, etc.
- `PointLight` and `DirectionalLight`: authored lighting.

## Parametric generators

- `Staircase`: solid ground-to-tread columns with exact step height/depth.
- `StepSweep`: side-by-side blocks spanning a numeric height range.
- `SlopeSweep`: side-by-side ramps spanning an angular range.
- `PillarGrid`: rectangular repeated obstacle field.

Generators deliberately remain geometric. Names such as “Stairworks” or “Surf Foundry” exist only in
map data; the engine does not know those concepts.

## Metadata

Maps declare `zones` and `materials`. Every generated geometry entity receives `AuthoredMapObject`
with its logical ID, zone and tags. Generic anchors receive `AuthoredMapMarker`. This is intended to
feed future debug overlays, regression assertions, editors and test automation without changing the
map format into gameplay code.

## Editing

With Bevy's `file_watcher` feature enabled, save the `.spacemap` while the executable is running. The
asset is reloaded and all geometry produced by that map is rebuilt. A malformed edit fails asset
loading rather than intentionally panicking the geometry compiler; validation checks duplicate IDs,
references, dimensions, finite transforms, convex prisms and generated-object limits.

The current playground adapter uses `player_spawn`, `portal_a` and `portal_b` marker kinds. Those
meanings are **not** part of the geometry runtime.
