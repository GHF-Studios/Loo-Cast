//! Build recursive portal views from camera paths and reusable render targets.
//!
//! ## Module map
//!
//! - `camera`: Recursive directed portal-camera placement.
//! - `path`: Describe the camera path through a recursive portal view.
//! - `targets`: Allocate and resize render targets for recursive portal views.
//! - `tree`: Recursive directed-face render tree.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

pub mod camera;
pub mod path;
pub mod targets;
pub mod tree;
