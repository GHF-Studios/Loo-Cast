//! Host-managed developer scripting workspace.
//!
//! Rhai receives no Bevy World or filesystem authority. Documents own draft,
//! candidate, and committed revisions; storage owns live/default files; the
//! console and editor are adapters over those operations.

mod commands;
mod document;
mod host;
mod storage;
mod ui;
mod workspace;

pub(crate) use commands::configure;
pub(crate) use ui::draw_script_workspace;
pub(crate) use workspace::DeveloperScriptWorkbench;
