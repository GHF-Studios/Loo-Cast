#![allow(dead_code)]

//! `core_mod` doc-only spec anchors for USF/capability/scale contracts.
//!
//! Related notes:
//! - `docs/glossary/Scale Contract Runtime Notes.md`
//! - `docs/glossary/USF Contract Runtime Boundary Notes.md`
//! - `docs/glossary/USF Runtime Evolution Lifecycle Notes.md`
//! - `docs/glossary/USF Instantiation Script Profile Notes.md`
//! - `docs/glossary/Capability Role and State Authority Notes.md`
//! - `docs/glossary/Rhai Generic Dispatch Policy Notes.md`
//! - `docs/glossary/Rhai Value Semantics and AccessCell Notes.md`
//! - `docs/glossary/USF Math Raw Model Foundation Notes.md`
//! - `docs/glossary/USF Position Stack and Overflow Policy Notes.md`

/// Anchor for scale declaration/support/realizer cardinality runtime concerns.
#[derive(Debug, Default, Clone, Copy)]
pub struct ScaleContractRuntimeAnchor;

/// Anchor for contract/runtime boundary notes tied to default first-party USF runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct UsfContractRuntimeBoundaryAnchor;

/// Anchor for definition-freeze to runtime-evolution flow notes.
#[derive(Debug, Default, Clone, Copy)]
pub struct UsfRuntimeEvolutionLifecycleAnchor;

/// Anchor for profile-gated script instantiation and context API usage notes.
#[derive(Debug, Default, Clone, Copy)]
pub struct UsfInstantiationScriptProfileAnchor;

/// Anchor for capability-role taxonomy and runtime state-authority split notes.
#[derive(Debug, Default, Clone, Copy)]
pub struct CapabilityRoleStateAuthorityAnchor;

pub mod rhai;
pub mod usf_math;
pub mod usf_pos;
