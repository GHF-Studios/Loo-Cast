//! Text-free world-space developer visualization.
//!
//! World Draw represents spatial meaning only. Text, labels, legends and
//! linguistic/numeric detail belong to Developer UI / inspection instead.

mod color;
mod frame;
mod render;

pub use color::{ColorRamp, ColorStop, ScalarRange};
pub use frame::{
    DrawDepth, DrawId, ScalarFieldMode, WorldDrawBatch, WorldDrawFrame, WorldPrimitive,
    WorldScalarField,
};

use bevy::prelude::*;

use super::DeveloperSet;

pub(super) fn configure(app: &mut App) {
    app.init_resource::<WorldDrawFrame>().add_systems(
        PostUpdate,
        frame::clear_world_draw_frame.in_set(DeveloperSet::ResolveFocus),
    );

    render::configure(app);
}
