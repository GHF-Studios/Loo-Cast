pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

struct Scale {
    magnitude: i8,
    name: &'static str,
    description: &'static str,
}

static SCALES: &[Scale] = &[
    Scale {
        magnitude: -36,
        name: "Change Potential Scale",
        description: "The universe begins as a field of change potential, serving as the raw canvas for all future developments."
    },
    Scale {
        magnitude: -35,
        name: "Change Partition Scale",
        description: "The change potential diversifies, splitting into two distinct potentials: one for state changes (leading to time) and another for relational changes (leading to space)."
    },
    Scale {
        magnitude: -34,
        name: "Dimensional Nexus Scale",
        description: "In regions of concentrated change potential, Dimensional Nexuses form. These nexuses are the seeds for the fabric of time and space."
    },
    Scale {
        magnitude: -33,
        name: "Topological Fluctuation Scale",
        description: "Dimensional Nexuses evolve into Topological Fluctuations, transient structures that allow patches of space and time to momentarily exist."
    },
    Scale {
        magnitude: -32,
        name: "Quantum Foam Scale",
        description: "An accumulation of Topological Fluctuations gives rise to Quantum Foam, a sponge-like structure featuring unstable, yet semi-persistent, space-time regions."
    },
    Scale {
        magnitude: -31,
        name: "Resonant Foam Mechanics Scale",
        description: "Regions of Quantum Foam can resonate, forming larger and more stable patches of coherent space-time."
    },
    Scale {
        magnitude: -30,
        name: "Elemental Energy Pathway Scale",
        description: "High-amplitude resonances in the foam create Elemental Energy Pathways, where energy starts to flow in a directed manner."
    },
    Scale {
        magnitude: -29,
        name: "Aetheric Nexus Scale",
        description: "Intersections of Elemental Energy Pathways coalesce into Aetheric Nexuses, marking the first signs of discrete energy manifestations."
    },
    Scale {
        magnitude: -28,
        name: "Essence Node Scale",
        description: "At this scale, Aetheric Nexuses evolve into Essence Nodes, discrete packets of energy carrying fundamental properties like charge, spin, and mass."
    },
    Scale {
        magnitude: -27,
        name: "Essence Manifestation Scale",
        description: "Clusters of Essence Nodes form, engaging in complex interactions that produce new types of energy relationships, similar to magnetic attraction or repulsion."
    },
    Scale {
        magnitude: -26,
        name: "Essence Flux Pattern Scale",
        description: "The Essence Nodes organize into larger structures, creating islands, oceans, and rivers of interacting essence."
    },
    Scale {
        magnitude: -25,
        name: "Essence Nexus Scale",
        description: "Complex webs of essence interactions form, leading to the emergence of stable structures that serve as the building blocks for elementary particles in the standard model of particle physics."
    },
    Scale {
        magnitude: -24,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -23,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -22,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -21,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -20,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -19,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -18,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -17,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -16,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -15,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -14,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -13,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -12,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -11,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -10,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -9,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -8,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -7,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -6,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -5,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -4,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -3,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -2,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: -1,
        name: "small",
        description: "description_here"
    },
    Scale {
        magnitude: 0,
        name: "normal",
        description: "description_here"
    },
    Scale {
        magnitude: 1,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 2,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 3,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 4,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 5,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 6,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 7,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 8,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 9,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 10,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 11,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 12,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 13,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 14,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 15,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 16,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 17,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 18,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 19,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 20,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 21,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 22,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 23,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 24,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 25,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 26,
        name: "big",
        description: "description_here"
    },
    Scale {
        magnitude: 27,
        name: "big",
        description: "description_here"
    },
];

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadUniverse>()
            .add_event::<ConfirmLoadedUniverse>()
            // Update Systems
            .add_systems(
                Update,
                handle_load_universe.run_if(in_state(AppState::Game)),
            );
    }
}
