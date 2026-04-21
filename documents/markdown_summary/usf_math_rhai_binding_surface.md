# USF Math Rhai Binding Surface

Purpose: define how the math sketch is documented and projected into script-facing APIs.

## Scope

- Applies to `documents/temp_stuff/TMP_usf_math_raw_model/*`.
- Applies to facade/binding design for Rhai exposure.
- Does not require immediate runtime implementation.

## Core Intent

- Math contracts are authored in Rust for precision, control, and performance ownership.
- Scripts consume facaded, monomorphized API surfaces.
- Rhai is the orchestration layer, not the owner of low-level math implementation.

## Runtime Anchors

- Bootstrap + typed entrypoints:
  - `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
  - `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
- Binding graph + metadata path:
  - `core_mod_api/src/rhai_binding/bind/*`
  - `core_mod_api/src/rhai_binding/meta/*`
  - `core_mod_api/src/rhai_binding/bridges/*`
- Script preprocessing:
  - `core_mod_api/src/rhai_binding/engine/preprocess.rs`

## Rust Contract -> Rhai Surface Pipeline

1. Define operation semantics in Rust core contracts (`*CoreOps`, `*FieldOps`, `*BridgeOps`).
2. Build facade methods that remove trait-level generic complexity.
3. Monomorphize facade signatures into bindable concrete functions/types.
4. Register into Rhai via bridge graph or explicit registration.
5. Expose script-call syntax aligned with ctx/capability model.

## Kind and Repr Semantics

- Kind (mathematical shape family):
  - `Scalar`
  - `Vector`
  - `Matrix`
  - `Tensor3`
  - `Tensor4`
  - `Quaternion`
  - `Translation`
  - `Rotation`
  - `Scale`
  - `Transform`
- Repr (representation regime):
  - `Usf`
  - `Normal`

Mechanisms:

- Mixed-repr inputs: `UsfOrNormal*`, `OneOf*`.
- Generic projection intent (pre-facade monomorphization): `Mode: op_mode::OpMode` (`op_mode::Mode<Kind, Repr>`).
- Operation-intrinsic mode variance: `op_policy::OpPolicy` with `OpPolicy::DeferToGlobal` as the default route.
- Invalid kind/repr selections for a concrete operation/backend: panic-fast guard clauses.

## Function Expansion Contract (Documentation Rule)

Every operation intended for scripting should document:

1. Concept summary:
   - Start with the core concept, not implementation detail.
2. Rust contract:
   - Exact parameter and return intent.
3. Kind/repr combinations:
   - Allowed and disallowed operand/output combinations.
4. Rhai surface intent:
   - Planned function/method name and call shape.
   - Which overload/facade specialization carries the selected kind/repr mode.
   - Whether operation-specific policy override is caller-controlled.
5. Panic contract:
   - Kind/repr/policy guard failures.
   - Mathematical undefined states (divide by zero, singular inverse, etc.).

## Rhai Surface Shape (Target)

- Prefer overloaded concrete facade bindings over script-side generic emulation.
- Avoid exposing raw generic trait surfaces directly to Rhai.
- Keep script syntax predictable:
  - method-like where object semantics are clear.
  - function-like for constructors/converters.
- Keep kind/repr controls explicit through overload/specialization selection, not runtime mode arguments.

## Example Expansion Pattern

- Rust contract:
- `fn determinant<Mode: OpMode>(&self, policy: OpPolicy) -> UsfOrNormalDecimalScalar`
- Rhai facade intent:
  - `mat.determinant()` for default policy route.
  - `mat.determinant(policy)` for explicit override paths.
  - where kind/repr projection is encoded by the selected overload/specialization.
  - where `policy` defaults to `DeferToGlobal` unless explicitly provided.
- Panic conditions:
  - invalid kind/repr projection selection
  - invalid operation-policy selection
  - unsupported backend combination

## Capability/Context Alignment

- Math exposure is contextual, not globally dumped.
- Facades should be available through typed ctx graphs and capability channels.
- Binding visibility should respect script type + schedule context rules.

## Status Rule

- This file defines target documentation/binding semantics.
- If runtime code diverges, record divergence explicitly and update this file with the chosen direction.
