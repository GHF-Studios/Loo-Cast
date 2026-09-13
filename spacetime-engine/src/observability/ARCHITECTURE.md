# Temporary telemetry shell

The original observability architecture has been superseded by the staged developer-tools redesign.

After Stage 4, developer controls, the F4 palette, focus, inspection and World Draw all live under `crate::devtools`. The old generic control graph/menu and Avian developer-control adapter are no longer declared modules and do not participate in the build or runtime.

The surviving `observability` module is temporary and owns only telemetry plus the `DebugArtifact` compatibility marker until Stage 5 moves diagnostics data to its final home.

**Canonical architecture and migration status:** [`../../DEVTOOLS_REDESIGN.md`](../../DEVTOOLS_REDESIGN.md)

Do not add new developer-facing functionality here. New inspection belongs in `devtools::InspectionFrame`; spatial visualization belongs in the text-free `devtools::draw` model; screen-space developer controls belong in `devtools::ui`.
