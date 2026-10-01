# Spacetime Engine profiling

Run the normal development configuration:

```sh
vapor run
```

Profile with Tracy:

```sh
vapor run --profiling
```

The Tracy build uses on-demand collection: instrumentation is present, but
trace collection starts only after a Tracy server/client connects.

Include allocation tracing:

```sh
vapor run --profiling-memory
```

Open `vapor-monitor` in another terminal for live logs/runtime telemetry.
Use Tracy for the detailed ECS/system timeline.
