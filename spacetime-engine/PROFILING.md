# Spacetime Engine profiling

Run the normal development configuration:

```sh
vapor run
```

Profile with Tracy:

```sh
vapor run --profiling
```

Include allocation tracing:

```sh
vapor run --profiling-memory
```

Open `vapor-monitor` in another terminal for live logs/runtime telemetry.
Use Tracy for the detailed ECS/system timeline.
