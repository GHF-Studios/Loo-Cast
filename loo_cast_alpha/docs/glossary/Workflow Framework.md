The Workflow Framework is the Rust-side orchestration layer for staged runtime work in the [[Runtime Substrate]].
It is responsible for lifecycle coordination (request, progression, completion/failure handling) across stage domains,
while stage logic itself remains scheduler-visible Bevy systems.
It orchestrates Rust-side runtime artifacts and [[Rust Capability]] contracts, not raw Rhai engine internals.

It treats `ECS`, `Render`, and `Async` as first-class workflow domains.
`EcsWhile` and `RenderWhile` are core iterative stage variants for non-async domains.

Scope boundary:

- workflow orchestration is Rust-side
- Rhai-side capability/domain exposure is handled by script-profile and binding surfaces, not by this framework

Implementation-facing notes:

- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)

Current run identity/concurrency draft is maintained in those premise notes.

#glossary
