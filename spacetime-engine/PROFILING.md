# Spacetime Engine profiling

## Premise

Performance work should begin with broad instrumentation that answers **where time
and memory go without editing individual gameplay systems**.

The default profiling path should make it cheap to answer:

- which frame was slow;
- which ECS systems and engine/render spans occupied its critical path;
- which work ran in parallel versus serially;
- which systems are consistently expensive versus merely spiky;
- where allocations happen;
- how much live inline ECS payload each component type represents;
- whether an optimization actually improved the same workload.

Manual timers and custom spans are a **drill-down tool**, not the baseline
instrumentation strategy.

## Layers

Spacetime uses complementary profiling layers because one mechanism cannot provide
high detail, low overhead, allocation attribution, and call-tree visibility at the
same time.

### 1. Always-on runtime diagnostics

`RuntimeDiagnosticsPlugin` samples low-frequency development telemetry:

- FPS, frame time, average frame time, and 1% low FPS;
- process/system CPU and memory;
- live entity, component-instance, and archetype counts;
- logical live inline ECS component payload, including the largest component types.

The ECS payload is:

```text
registered component layout size × live component instances
```

This is useful for answering questions such as "are voxel cells or transforms
dominating the live ECS payload?" but it is **not process RAM**. It excludes:

- heap allocations owned by component values (`Vec`, `String`, maps, boxes, etc.);
- ECS spare capacity, table/sparse-set metadata, entity metadata, and allocator overhead;
- resources and assets;
- renderer/driver/GPU allocations;
- memory mapped files and other OS-managed memory.

Use the process-memory diagnostic for the whole-process view and Tracy allocation
capture when allocation behavior needs attribution.

### 2. Tracy timeline — primary deep profiler

Bevy already instruments ECS systems, render work, engine internals, and user
spans through `tracing`. The Spacetime profiling features expose that instrumentation
rather than manually wrapping every system.

From the workspace root:

```sh
cargo run -p spacetime-engine --profile profiling --features profiling-tracy
```

Keep tracing at `info` or more verbose. A `LogPlugin` filter or compile-time
`tracing` max-level that removes `info` spans will also remove profiling data.
Use one trace backend per profiling build.

The custom `profiling` Cargo profile inherits release optimizations while retaining
debug symbols.

For allocation events as well:

```sh
cargo run -p spacetime-engine --profile profiling --features profiling-tracy-memory
```

For Bevy's more expensive detailed trace events:

```sh
cargo run -p spacetime-engine --profile profiling \
  --features profiling-tracy,profiling-detailed
```

Do not use detailed tracing by default. It exists for focused investigations.

To identify the Tracy version compatible with the Rust Tracy client selected by
the dependency graph:

```sh
cargo tree -p spacetime-engine --features profiling-tracy | grep tracy
```

For cleaner captures, prefer Tracy's command-line recorder over running the full
Tracy GUI alongside the game:

```sh
tracy-capture -o baseline.tracy
```

Start the capture first, then launch the profiling build.

### 3. Chrome trace / Perfetto — portable secondary backend

```sh
cargo run -p spacetime-engine --profile profiling --features profiling-chrome
```

After the application exits, Bevy writes a Chrome-tracing JSON capture. Open it in
Perfetto. This is useful when Tracy is unavailable or when a shareable browser
trace is more convenient.

### 4. Sampling / flame graphs — inside a hot span

Trace spans answer **which system/span is expensive**. A sampling profiler answers
**which functions/instructions inside that span are expensive**.

Once a hot system has been identified, use `perf`/`cargo flamegraph` (or the
platform-equivalent sampling profiler) against the optimized profiling build. Do
not start here unless the system-level timeline is insufficient.

## Reading a capture correctly

### Wall time is not summed system time

Systems execute in parallel. Two 4 ms systems overlapping completely consume about
4 ms of frame wall time, not 8 ms. Optimize the critical path and synchronization
constraints, not a naive sum of all span durations.

### A long span is not automatically CPU work

CPU spans can include waiting. In rendering especially, several prepare systems
finishing late together can indicate GPU/queue backpressure rather than expensive
CPU instructions. Switch to GPU tooling once the CPU trace points there.

### Profile optimized builds

Debug/dev timings can be dominated by unoptimized Rust rather than the algorithm
you are trying to evaluate. Use the `profiling` profile for measurements intended
to drive optimization decisions.

### Instrumentation has observer cost

Tracy tracing changes timing slightly; allocation tracing changes it more; detailed
tracing can change it substantially. Use the least intrusive mode that answers the
question and compare like-for-like captures.

### Warm-up is a different workload

Asset loading, shader compilation, cache population, world generation, and first
use initialization can dominate early frames. Measure startup separately from
steady-state gameplay instead of mixing the two distributions.

### Frame caps can hide headroom

VSync or an explicit frame limiter can turn spare CPU time into waiting. A 16.67 ms
frame at 60 Hz is not proof that 16.67 ms of CPU work occurred.

## Standard optimization loop

1. Define a repeatable scenario and capture a baseline.
2. Find slow frames and the spans on their critical path.
3. Inspect the system/span distribution, not only one frame.
4. If the span is still too coarse, add a small number of custom `info_span!`
   regions inside that already-proven hotspot.
5. If allocation churn is suspected, repeat with `profiling-tracy-memory`.
6. Change one meaningful thing.
7. Re-run the same scenario and compare captures.
8. Keep the change only if the relevant metric actually improved without moving
   the bottleneck somewhere worse.

## Planned extensions

The foundation intentionally does not fake precision that is not available yet.
Useful follow-ups, added when real profiling pressure justifies them, are:

- ECS storage capacity/fragmentation rather than logical live payload only;
- resource and asset memory adapters;
- retained heap ownership/tagging when allocation-event attribution is insufficient;
- GPU timestamp/counter integration and renderer-specific views;
- capture metadata (scenario, commit, settings, hardware) for reproducible comparisons;
- optional in-editor history graphs and spike bookmarking.

The governing rule stays simple: **broad automatic measurement first; bespoke
instrumentation only where the broad profile proves it is needed.**
