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
        name: "Neutrino Emergence Scale",
        description: "The universe's essence coalesces into stable, elementary particles known as neutrinos. These particles serve as the first stable seeds for the next layers of complexity."
    },
    Scale {
        magnitude: -23,
        name: "Neutrino-Essence Synergy Scale",
        description: "Neutrinos begin to interact with the essence flux patterns, giving rise to a menagerie of unstable neutrino variants. These rare and elusive variants offer glimpses into higher complexities."
    },
    Scale {
        magnitude: -22,
        name: "High Energy Neutrino Cataclysm Scale",
        description: "Neutrinos cluster into ultra-high-energy forms, teetering on the brink of catastrophic collapse. Their existence is a precarious balancing act, a dance on the edge of annihilation."
    },
    Scale {
        magnitude: -21,
        name: "Proto-Quark Genesis Scale",
        description: "Quantum tunneling events cause high-energy neutrinos to collapse into proto-quarks. These entities are ephemeral, quickly reverting back to neutrinos, yet they open the door for the birth of a new class of particles."
    },
    Scale {
        magnitude: -20,
        name: "Quark-Chromodynamic Inception Scale",
        description: "The first top and bottom quarks emerge as coherent structures within proto-quarks. Their stable existence marks the onset of Quantum Chromodynamics and sets the stage for the next level of particle interaction."
    },
    Scale {
        magnitude: -19,
        name: "Weak Force Enigma Scale",
        description: "An enigmatic process allows the weak force to accumulate and coalesce simple quarks into more complex up and down quarks. These new quarks are unstable but serve as a crucial stepping stone."
    },
    Scale {
        magnitude: -18,
        name: "Volatile Quark Scale",
        description: "Quarks occasionally reach critical densities, forming incredibly unstable, ultra-high-energy quarks known as charm quarks. These volatile entities have the potential to disrupt the essence flow across vast regions."
    },
    Scale {
        magnitude: -17,
        name: "Strange Quark Anomaly Scale",
        description: "The universe defies its own odds. Charm quarks stabilize into strange quarks, the universe's most volatile substance. Even a single strange quark poses a threat to the stability of existence itself."
    },
    Scale {
        magnitude: -16,
        name: "Gluonic Revolution Scale",
        description: "A radical shift occurs as essence manifests into gluons, carriers of the strong force. These particles rewrite the rules of interaction, transforming quark relationships into complex webs."
    },
    Scale {
        magnitude: -15,
        name: "Baryonic Emergence Scale",
        description: "A critical mass is achieved as quarks coalesce into stable entities known as baryons. These baryons act as the cornerstones for the next wave of material complexity."
    },
    Scale {
        magnitude: -14,
        name: "Nucleonic Foundation Scale",
        description: "As baryons find stability, they pave the way for the emergence of simple atomic nuclei. It's a fleeting moment that marks the edge of a new frontier in complexity."
    },
    Scale {
        magnitude: -13,
        name: "Electron Genesis Scale",
        description: "The strong force meets its limits, leading to the decay of complex nuclei. This decay gives birth to a new form of particle, the electron, heralding the dawn of electromagnetic interactions."
    },
    Scale {
        magnitude: -12,
        name: "Photon Emergence Scale",
        description: "Hypercomplex interactions between several layers of reality lead to the emergence of incredibly high energy photons, the carriers of the electromagnetic force. From here on, the electromagnetic force mostly governs the interactions between different entities of various different types."
    },
    Scale {
        magnitude: -11,
        name: "Photonic Void Scale",
        description: "This scale is very unique, in that almost every entity known is either vastly larger or smaller then this scale. This is a realm inhabited mostly by very high energy photons, dangerous space-time vortices and many other serious threats to structured existence."
    },
    Scale {
        magnitude: -10,
        name: "Atomic Inception Scale",
        description: "Electrons and nuclei combine to form atoms, the building blocks of matter as we know it. Atoms are the first entities to be able to form stable structures, like molecules, crystals, and even life."
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
