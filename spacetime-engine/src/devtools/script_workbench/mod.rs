//! Host-managed developer scripting workspace.
//!
//! Rhai receives no Bevy World or filesystem authority. Documents own draft,
//! candidate, and committed revisions; storage owns live/default files; the
//! console and editor are adapters over those operations.
//!
//! ## Module map
//!
//! - `commands`: Developer console ingress for script workspace operations.
//! - `document`: Target contracts and draft-to-committed document lifecycle.
//! - `host`: Bounded Rhai host configuration and script logging.
//! - `storage`: Default/live script roots, materialization, discovery, and path validation.
//! - `ui`: Script explorer and editor surface embedded in the developer console.
//! - `workspace`: Script workspace, live roots, open tabs, and document operations.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod commands;
mod document;
mod host;
mod storage;
mod ui;
mod workspace;

pub(crate) use commands::configure;
pub(crate) use document::ScriptTarget;
pub(crate) use ui::draw_script_workspace;
pub(crate) use workspace::DeveloperScriptWorkbench;
