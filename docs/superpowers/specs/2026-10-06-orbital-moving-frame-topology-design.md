# Orbital moving-frame topology — tranche 1
<!-- loo-cast-orbital-architecture-topology-v2 -->

The reusable primitive is a moving semantic kinematic frame. A celestial voxel world ("voxelet") is one capability anchored to that frame; voxels do not gain independent world-space motion authority.

Contracts in this tranche:
- canonical motion carries SI linear/angular state, epoch, and typed authority;
- `UsfKinematicFrameState` exposes pose/motion and rigid-point velocity `v + ω × r`;
- pure `KeplerianElements`/`OrbitalStateVector` can be used by authoritative rails or later by non-authoritative trajectory planning;
- Earth and Moon are ordinary semantic celestial bodies; Moon advances the same canonical state seen by gravity/navigation/voxels;
- dense celestial worlds preserve body-local cache identity under translation and republish render/collision/capability pose;
- spacecraft orbital telemetry is relative to a live moving primary;
- celestial landmarks resolve live body positions rather than spawn-time centers.

Dense voxel limitation: the current materialization lattice is canonical-axis aligned. Translation can preserve keys/caches exactly. Arbitrary body rotation cannot yet rotate that lattice. Orientation/angular state is represented now, but a changed celestial orientation retires/rebuilds disposable scale worlds until addressing/projection becomes genuinely frame-local.

Ownership: #26 constructs semantic bodies/provenance; #60 owns motion/model regimes; #48 owns the integrated player-facing orbital program; #49 keeps exactly-once contact authority. Prediction never becomes motion authority, and patched-conic primary selection never becomes semantic parentage.

Non-goals: Map View UI, maneuver gizmos, SOI switching, navball rendering, MechJeb, time warp, N-body authority, rotating dense voxelets, and voxelet-vs-voxelet contact are downstream tranches.
