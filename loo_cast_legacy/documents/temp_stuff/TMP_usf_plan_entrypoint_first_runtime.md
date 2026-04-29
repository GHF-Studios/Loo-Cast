# TEMP Plan: Entrypoint-First USF Runtime Platform

Date: 2026-04-03  
Status: active execution track (`partially_validated`)

## Goal

Make script entrypoints the primary USF authoring and orchestration interface, with deterministic typed ctx injection and strict bootstrap validation.

## Target End State

1. Every USF content concept is registered only via typed script entrypoints.
2. Entry points are explicit contracts (`script type`, `function name`, `signature`, `ctx type`).
3. Bootstrap is deterministic and fail-fast for signature mismatch or missing entrypoint.
4. Runtime systems consume immutable compiled registries, not ad-hoc script state.

## Architecture Shape

1. `ScriptTypeContract`
   - script type id
   - expected file suffix
   - expected function name
   - expected parameter shape
   - ownership rules (mod/modpack/global)
2. `EntrypointDescriptor`
   - absolute script path
   - script type
   - owner id
   - declared entrypoint function
   - diagnostics metadata (line/function)
3. `EntrypointRegistry`
   - deterministic load order
   - validated descriptors
   - typed ctx factory paths
4. `BootstrapReport`
   - per-script success/failure
   - normalized error codes
   - counters for skipped/invalid/loaded entries

## Execution Steps

1. Contractization
   - define a single source of truth for script type contracts.
   - remove duplicated signature assumptions scattered in bootstrap code.
2. Loader normalization
   - scan script tree by contract, not by ad-hoc branching.
   - bind each file to exactly one script type.
3. Validation hardening
   - assert one-entity-per-file where applicable.
   - assert entrypoint function name + parameter count.
   - assert owner resolution (mod/modpack/global) before execution.
4. Typed ctx lifecycle
   - instantiate ctx by script type through one factory path.
   - keep ctx scope limited to entrypoint execution.
   - emit deterministic diagnostics for ctx init failures.
5. Registry compilation
   - compile entrypoint-produced declarations into immutable snapshots.
   - reject partial-invalid snapshots.
6. Diagnostics
   - standardize bootstrap error messages and codes.
   - include script path + contract + expected vs actual signature.

## Acceptance Criteria

1. Invalid script files fail startup with deterministic, actionable diagnostics.
2. Removing a required entrypoint in any USF script type fails startup immediately.
3. Load order is deterministic across restarts.
4. Runtime behavior is unchanged when script file system ordering changes.

## Risks and Mitigations

1. Risk: hidden legacy fallback behavior survives in old code paths.
   - Mitigation: explicitly delete fallback branches and add startup assertions.
2. Risk: typed ctx drift between docs and runtime.
   - Mitigation: generate docs table from the same contract metadata used at runtime.
3. Risk: ownership confusion between mod and modpack declarations.
   - Mitigation: enforce owner resolution before entrypoint invocation.

## Sequencing Notes

1. Execute before broader capability expansion to avoid rework.
2. Keep compatibility shims short-lived and marked for deletion in same phase.
