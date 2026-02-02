use crate::bevy::ecs::query::QueryFilter;
use crate::bevy::picking::pointer::PointerId;
use crate::bevy::prelude::*;
use std::fmt;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

use crate::picking::constants::{DIEGETIC_MOUSE_POINTER_ID, META_MOUSE_POINTER_ID};

use super::statics::START_TIME;

#[derive(Reflect)]
pub struct ShortTime;
impl FormatTime for ShortTime {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let elapsed = START_TIME().elapsed();
        let millis = elapsed.subsec_millis();
        let secs = elapsed.as_secs() % 60;
        let mins = (elapsed.as_secs() / 60) % 60;
        write!(w, "T+ {:02}m:{:02}s.{:03}ms", mins, secs, millis)
    }
}

/// Marker trait for distinguishing between ontological contexts,
/// such as diegetic (in-world) and meta (UI, overlays, etc.).
///
/// This allows systems to be generically parameterized over which
/// "layer of reality" they operate in, without duplicating logic.
/// Ontological contexts can control aspects like pointer identity,
/// query filters, visibility rules, and more.
///
/// Typically implemented using zero-sized structs (ZSTs).
pub trait OntologicalContext {
    /// Filter used to distinguish which entities belong to this context.
    type SpriteOntologyFilter: QueryFilter;

    fn name() -> &'static str;

    /// PointerId to use for this context.
    fn pointer_id() -> PointerId;
}

/// Represents the in-world, player-visible reality (e.g., game world).
#[derive(Reflect)]
pub struct Diegetic;

/// Represents extra-diegetic overlays like UI, debug tools, menus.
#[derive(Reflect)]
pub struct Meta;

impl OntologicalContext for Diegetic {
    type SpriteOntologyFilter = Without<super::components::Meta<Sprite>>;

    fn name() -> &'static str {
        "Diegetic"
    }

    fn pointer_id() -> PointerId {
        DIEGETIC_MOUSE_POINTER_ID
    }
}

impl OntologicalContext for Meta {
    type SpriteOntologyFilter = With<super::components::Meta<Sprite>>;

    fn name() -> &'static str {
        "Meta"
    }

    fn pointer_id() -> PointerId {
        META_MOUSE_POINTER_ID
    }
}
