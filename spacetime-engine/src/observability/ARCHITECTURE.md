# Legacy observability shell

The original observability architecture has been superseded by the staged developer-tools redesign.

The surviving `observability` module is temporary and currently owns only:

- the legacy F4 control graph/menu, pending Stage 4;
- telemetry and the `DebugArtifact` compatibility marker, pending Stage 5;
- temporary control adapters for developer visualizations that have already moved to `crate::devtools`.

Focus, inspection, and world drawing now live under `crate::devtools`. The old `DebugFrame`, world-text path, legacy Inspector, and debug context were removed in Stage 3B.

**Canonical architecture and migration status:** [`../../DEVTOOLS_REDESIGN.md`](../../DEVTOOLS_REDESIGN.md)

Do not add new developer-facing functionality to this legacy module. New inspection belongs in `devtools::InspectionFrame`; spatial visualization belongs in the text-free `devtools::draw` model; Stage 4 will replace the remaining generic control graph.
