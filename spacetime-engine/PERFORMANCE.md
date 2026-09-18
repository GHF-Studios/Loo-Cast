# Performance discipline

Hot-path rules:

- Do no work when relevant state did not change.
- Cache derived facts at mutation boundaries instead of rediscovering them every frame.
- Share immutable snapshots; avoid deep clones in queues and worker handoff.
- Reuse scratch buffers and collection capacity.
- Bound work per frame and rank only work that can actually start.
- Avoid unconditional ECS writes; Bevy change ticks are work too.
- Diagnostics/devtools are pay-for-play: no world scans or per-item logging in ordinary runtime.
- Keep expensive pure derivation off the main thread; publish results in bounded batches.

Measure first with `vapor run --profiling`; use `vapor-monitor` for live semantic metrics.
