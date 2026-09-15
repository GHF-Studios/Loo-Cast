# Editor gizmos and manipulation

A **gizmo** is a rich contextual observation/manipulation tool. Viewport handles
are one possible part of a gizmo, not the definition of one. A Thermal gizmo may
be mostly state, visualization and actions; a Transform gizmo naturally includes
viewport interaction and value editing.

## Selection and context

There is one canonical `DeveloperFocus` target model. Hover, editor selection and
pinning are acquisition modes for the same concrete/semantic entity pair.
`StructureSelection` then refines that entity focus to a stable `StructureItemId`.

Consequences:

- Hierarchy and Game-view picking choose the concrete entity focus.
- Structure chooses the meaningful semantic part.
- Semantic Inspector and Gizmos consume that refinement.
- ECS Inspector remains raw whole-entity reflection.
- A non-spatial Hierarchy selection is valid focus even though it has no
  `FocusHit`; world-projected UI simply declines to render for that target.

## Transform gizmo

The Transform gizmo is project-owned rather than an adapter over Bevy's mutually
exclusive transform-gizmo modes. Translate, Rotate and Scale affordances are all
visible simultaneously:

- axis arrows: translation;
- axis rings: rotation;
- inner axis handles: scale;
- World/Local is one Transform-gizmo setting, not three separate editor tools;
- generic scale handles remain local-axis because `Transform::scale` itself is local data; a true world-space scale edit requires richer decomposition/authority semantics.

The gizmo renders through the existing `WorldDrawFrame`, so there is no private
Bevy layer-15 overlay camera or projection compatibility shim.

### Visibility vs authority

Having `Transform` makes the Transform gizmo **applicable and observable**. It does
not grant mutation authority.

`EditorTransformWritable` explicitly enables the generic direct-runtime write
path. Parented transforms remain read-only in that generic path because local vs
world authoring semantics need a domain-aware adapter. Generated, simulated,
asset-authored or otherwise derived transforms should likewise commit through
their real authority rather than attaching the marker casually.

The semantic Transform inspection reports the same access distinction through
`InspectAccess`.

### Interaction lifecycle

- Handle hit testing is performed in the primary game viewport's screen space.
- Active drags claim generic `InputFocus`.
- Transform edits run in `Update`, before Bevy's normal `PostUpdate` transform
  propagation, avoiding one-frame `GlobalTransform` lag.
- Escape cancels an active drag and restores the pre-drag Transform.
- Releasing the pointer commits the current runtime edit. Proper authoring
  transactions/undo remain a later roadmap phase.

## Domain gizmos

Domain-specific gizmos should live beside their domain and consume shared focus,
inspection/value-widget, view and input contracts. They may contribute any useful
combination of:

- structured state;
- editable values when legal;
- viewport drawing;
- viewport interaction;
- contextual actions;
- visualization controls.

A gizmo must never manufacture mutation authority. If a domain requires commands,
validated setters, transactions, asset edits or another persistence route, its
gizmo writes through that mechanism.

## Thermal gizmo

Thermal is the deliberately different second concrete gizmo. Selecting the
`Thermal` Structure item exposes aggregate/material/combustion state, validated
runtime parameters, explicit heat/cool/reset actions, and a viewport visualization
of active thermal samples plus combustible heat-transfer radius. Runtime facts
such as temperature, fuel and current combustion remain observational.

The actions route through `ThermalImpulse`; the editor never inserts derived
`Combustion` state or writes temperature directly.

## Why there is still no universal gizmo/provider registry

Transform + Thermal now exercise the shared focus, Structure, inspection, action
and value-widget contracts. That is enough to stabilize those concrete APIs, but
still not enough evidence for one catch-all `EditorModule` or giant callback
registry. Registration/provider ergonomics should be extracted narrowly from the
next few domains rather than invented wholesale.

See [`ROADMAP.md`](ROADMAP.md) for the broader work queue.
