// Modules


// Local imports


// Internal imports


// External imports


// Static variables
pub static SCALES: &[Scale] = &[
    Scale {
        level: 64,
        order_of_magnitude: -36,
        name: "Change Potential Scale",
        description: "At this scale, the universe presents as a field of 'change potential,' where localized hotspots govern both state changes (time) and relational changes (space)."
    },
    Scale {
        level: 63,
        order_of_magnitude: -35,
        name: "Change Partition Scale",
        description: "When the field of change potential is observed at a larger scale, two distinct potentials for state and relational changes become distinguishable."
    },
    Scale {
        level: 62,
        order_of_magnitude: -34,
        name: "Dimensional Nexus Scale",
        description: "At a larger observational scale, foundational anchors in the fabric of time and space, known as 'Dimensional Nexuses,' emerge due to the concentration of partitioned potentials."
    },
    Scale {
        level: 61,
        order_of_magnitude: -33,
        name: "Topological Fluctuation Scale",
        description: "When viewed at an expanded scale, small-scale distortions known as 'Topological Fluctuations' become noticeable. These create transient patches of space and time and emerge from the aggregation of Dimensional Nexuses."
    },
    Scale {
        level: 60,
        order_of_magnitude: -32,
        name: "Quantum Foam Scale",
        description: "At this scale, 'Quantum Foam' becomes evident, manifesting as unstable yet semi-persistent regions of space-time. These regions emerge due to the averaging out of Topological Fluctuations."
    },
    Scale {
        level: 59,
        order_of_magnitude: -31,
        name: "Resonant Foam Mechanics Scale",
        description: "When Quantum Foam is observed on a broader scale, resonance phenomena stabilize certain regions into coherent patches of space-time."
    },
    Scale {
        level: 58,
        order_of_magnitude: -30,
        name: "Elemental Energy Pathway Scale",
        description: "At a larger scale, 'Elemental Energy Pathways' become discernible within the Quantum Foam, guiding the flow of energy due to the prevalence of resonant regions."
    },
    Scale {
        level: 57,
        order_of_magnitude: -29,
        name: "Aetheric Nexus Scale",
        description: "When Elemental Energy Pathways are abundant, their intersections give rise to 'Aetheric Nexuses,' marking the first instances of discrete, localized energy packets."
    },
    Scale {
        level: 56,
        order_of_magnitude: -28,
        name: "Essence Node Scale",
        description: "At this scale, energy condenses into 'Essence Nodes' within Aetheric Nexuses due to the high concentration of localized energy packets. These nodes are endowed with properties like charge, spin, and mass."
    },
    Scale {
        level: 55,
        order_of_magnitude: -27,
        name: "Essence Manifestation Scale",
        description: "When a significant number of Essence Nodes are present, they cluster and interact, giving rise to various energy relationships, akin to magnetic fields or gravitational pull."
    },
    Scale {
        level: 54,
        order_of_magnitude: -26,
        name: "Essence Flux Pattern Scale",
        description: "At a larger scale, the interactions among Essence Nodes average out to form 'Essence Flux Patterns,' large-scale structures resembling islands or rivers of interacting energy."
    },
    Scale {
        level: 53,
        order_of_magnitude: -25,
        name: "Essence Nexus Scale",
        description: "When Essence Flux Patterns become prevalent, they give rise to 'Essence Nexuses,' complex webs that serve as the structural blueprints for elementary particles."
    },
    Scale {
        level: 52,
        order_of_magnitude: -24,
        name: "Neutrino Emergence Scale",
        description: "At this scale, stable 'neutrinos' make their first appearance, emerging as the most stable energy configurations within the high-density field of Essence Nexuses."
    },
    Scale {
        level: 51,
        order_of_magnitude: -23,
        name: "Neutrino-Essence Synergy Scale",
        description: "As the observational scale expands, a multitude of unstable neutrino variants form. These variants acquire unique characteristics through direct energy exchanges with Essence Flux Patterns."
    },
    Scale {
        level: 50,
        order_of_magnitude: -22,
        name: "High Energy Neutrino Cataclysm Scale",
        description: "When the field of view broadens further, ultra-high-energy neutrinos form precarious clusters, teetering on the edge of annihilation. These volatile formations originate from the collision and fusion of unstable neutrino variants."
    },
    Scale {
        level: 49,
        order_of_magnitude: -21,
        name: "Proto-Quark Genesis Scale",
        description: "At this expanded scale, fleeting 'proto-quarks' sporadically crystallize from the disintegration of high-energy neutrinos, serving as potential precursors to more stable forms of matter."
    },
    Scale {
        level: 48,
        order_of_magnitude: -20,
        name: "Quark-Chromodynamic Inception Scale",
        description: "With the zoom extended, 'top' and 'bottom' quarks stabilize, acting as the foundational elements for strong-force interactions. These quarks manifest when proto-quarks reach a critical stability threshold."
    },
    Scale {
        level: 47,
        order_of_magnitude: -19,
        name: "Weak Force Enigma Scale",
        description: "As the scale broadens, 'up' and 'down' quarks intermittently emerge, adding an additional layer of complexity. These forms materialize during transitional phases where top and bottom quarks exchange energy."
    },
    Scale {
        level: 46,
        order_of_magnitude: -18,
        name: "Volatile Quark Scale",
        description: "At this wider scale, critical densities of quarks coalesce to give birth to 'charm quarks,' which are ultra-high-energy and highly unstable. These charm quarks result from the collision-induced energy spikes among simpler quarks."
    },
    Scale {
        level: 45,
        order_of_magnitude: -17,
        name: "Strange Quark Anomaly Scale",
        description: "Upon further scale expansion, 'strange quarks' occasionally stabilize from their charm quark precursors, marking them as some of the most unpredictable forms of matter."
    },
    Scale {
        level: 44,
        order_of_magnitude: -16,
        name: "Gluonic Revolution Scale",
        description: "At this level of observation, 'gluons' emerge as the mediators of the strong force, facilitating the intricate dance of quark interactions. These gluons appear when charm and strange quarks reach a critical density, necessitating a new form of interaction."
    },
    Scale {
        level: 43,
        order_of_magnitude: -15,
        name: "Baryonic Emergence Scale",
        description: "When viewed at a more expansive scale, quarks bind into 'baryons,' providing the cornerstone for more elaborate structures. These baryons form through gluon-mediated attractions between quarks."
    },
    Scale {
        level: 42,
        order_of_magnitude: -14,
        name: "Nucleonic Foundation Scale",
        description: "At this broader scale, baryonic aggregations stabilize to form 'nuclei,' which serve as the backbone for atomic structures. These nuclei establish themselves through resonant interactions among baryons."
    },
    Scale {
        level: 41,
        order_of_magnitude: -13,
        name: "Electron Genesis Scale",
        description: "As the scale continues to widen, 'electrons' emerge from specific decay pathways within the stabilized nuclei, adding a new dimension to the universe's complexity."
    },
    Scale {
        level: 40,
        order_of_magnitude: -12,
        name: "Photon Emergence Scale",
        description: "At this more expansive scale, 'photons' manifest as the dominant energy carriers, stemming from the constant conversion of electron and nuclear energies into electromagnetic radiation."
    },
    Scale {
        level: 39,
        order_of_magnitude: -11,
        name: "Photonic Void Scale",
        description: "As the observational scale further widens, a void-like realm emerges, dominated by high-energy photons and occasional space-time anomalies. This void arises from the unbalanced propagation of photons, leading to a reduced presence of structured matter."
    },
    Scale {
        level: 38,
        order_of_magnitude: -10,
        name: "Atomic Inception Scale",
        description: "At this broadened scale, electrons and nuclei engage in dynamic unions, forming the first rudimentary 'atoms.' These atomic structures materialize through stable orbital resonances between electrons and nuclei, providing the foundation for more complex forms of matter."
    },
    Scale {
        level: 37,
        order_of_magnitude: -9,
        name: "Quantum Mechanical Scale",
        description: "At this scale, quantum mechanics dominates, allowing for the electron cloud configurations that give rise to the rich variety of chemical elements. Both the wave and particle natures of matter are significant here."
    },
    Scale {
        level: 36,
        order_of_magnitude: -8,
        name: "Quantum Resonance Scale",
        description: "At this scale, the principles of quantum resonance dictate both chemical bonds and electron behavior in condensed matter physics. These quantized states are foundational to phenomena like superconductivity and chemical reactivity."
    },
    Scale {
        level: 35,
        order_of_magnitude: -7,
        name: "Geometric and Kinetic Harmony Scale",
        description: "The spatial arrangement of atoms and the kinetics of their reactions find a harmonious balance at this scale. This is pivotal for the formation of complex molecules in both living systems and inorganic materials."
    },
    Scale {
        level: 34,
        order_of_magnitude: -6,
        name: "Unified Forces Scale",
        description: "Various forces including van der Waals, hydrogen bonding, and even ionic interactions become significant, demonstrating a unity of forces that govern the behavior of matter in both biological systems and engineered materials."
    },
    Scale {
        level: 33,
        order_of_magnitude: -5,
        name: "Interface and Cohesion Scale",
        description: "At this level, the principles of interface interactions and cohesion become universally significant, governing the behavior of colloidal systems, biological membranes, and even geological formations like sediment layers."
    },
    Scale {
        level: 32,
        order_of_magnitude: -4,
        name: "Microdynamic Systems Scale",
        description: "This scale is governed by the principles of motion and flow, whether it's the motility of cells in a biological context or the principles of microfluidics that govern small-scale engineered systems."
    },
    Scale {
        level: 31,
        order_of_magnitude: -3,
        name: "Structural Integrity Scale",
        description: "The structural integrity of systems, whether biological or mechanical, becomes crucial. This includes the arrangement of cells in tissues as well as the alignment of crystalline structures in metals and other materials."
    },
    Scale {
        level: 30,
        order_of_magnitude: -2,
        name: "Surface and Fluid Interaction Scale",
        description: "At this scale, interactions between surfaces and fluids manifest in ways that are critical across scientific disciplines. Whether it's capillary action in plants or wicking in engineered materials, the principles at this scale govern a range of phenomena."
    },
    Scale {
        level: 29,
        order_of_magnitude: -1,
        name: "Systems Dynamics Scale",
        description: "The principles of dynamics come into play at this scale, governing everything from biomechanical systems like human movement to the flow dynamics in geological formations like rivers or lava flows. The laws of motion and energy conservation are universally applicable."
    },
    Scale {
        level: 28,
        order_of_magnitude: 0,
        name: "Equilibrium and Stability Scale",
        description: "This scale is dominated by the interplay of forces that bring systems into equilibrium. These principles are not just limited to gravitational and electromagnetic forces but also include thermal and chemical equilibria that are foundational in various domains."
    },
    Scale {
        level: 27,
        order_of_magnitude: 1,
        name: "Structural Cohesion Scale",
        description: "At this scale, the integrity of both biological and man-made structures becomes crucial. The principles of mechanical engineering find parallels in the biological world through the structural design of organisms and their habitats."
    },
    Scale {
        level: 26,
        order_of_magnitude: 2,
        name: "Environmental Systems Scale",
        description: "Larger ecological, geophysical, and meteorological phenomena manifest at this scale. The principles that govern these large systems, such as energy transfer and mass flow, are consistent across both living and non-living entities."
    },
    Scale {
        level: 25,
        order_of_magnitude: 3,
        name: "Geospatial Systems Dynamics Scale",
        description: "At this scale, the dynamics of physical systems manifest through tectonic movements, atmospheric patterns, and resource cycles. These phenomena are fundamentally driven by principles of energy conservation and mass transfer, applicable across a variety of planetary landscapes."
    },
    Scale {
        level: 24,
        order_of_magnitude: 4,
        name: "Fluid and Thermal Dynamics Scale",
        description: "Principles of fluid dynamics and thermodynamics govern the behavior of atmospheres and subsurface oceans across celestial bodies."
    },
    Scale {
        level: 23,
        order_of_magnitude: 5,
        name: "Ecological and Geological Pattern Scale",
        description: "Complex patterns emerge in both living and non-living systems at this scale, from the distribution of biomes to the arrangement of geological formations. These patterns are shaped by underlying principles of resource allocation and energy gradients."
    },
    Scale {
        level: 22,
        order_of_magnitude: 6,
        name: "Planetary Systems Interaction Scale",
        description: "Gravitational and electromagnetic interactions between planetary bodies shape phenomena from orbital mechanics to tectonic activity across planetary systems."
    },
    Scale {
        level: 21,
        order_of_magnitude: 7,
        name: "Celestial Formation and Evolution Scale",
        description: "Principles like mass aggregation and angular momentum become significant, shaping the formation of various celestial bodies and early stages of star formation."
    },
    Scale {
        level: 20,
        order_of_magnitude: 8,
        name: "Core Dynamics and Energetics Scale",
        description: "The internal dynamics of celestial bodies, including their cores and mantles, become crucial at this scale. Core processes like magnetic field generation and thermal flows are central to the stability and life cycles of these bodies."
    },
    Scale {
        level: 19,
        order_of_magnitude: 9,
        name: "Radiative and Magnetic Phenomena Scale",
        description: "Surface and atmospheric phenomena of stars and celestial bodies are defined by principles like magnetohydrodynamics and thermal radiation."
    },
    Scale {
        level: 18,
        order_of_magnitude: 10,
        name: "Orbital Resonance and Stability Scale",
        description: "Stability of planetary and celestial orbits is governed by principles such as gravitational forces, orbital resonances, and tidal interactions."
    },
    Scale {
        level: 17,
        order_of_magnitude: 11,
        name: "Local Stellar Phenomena Scale",
        description: "This scale includes phenomena like asteroid belts, protoplanetary disks, and local interstellar clouds. It's governed by principles like gravitational interaction, radiation pressure, and mass distribution."
    },
    Scale {
        level: 16,
        order_of_magnitude: 12,
        name: "Star System Boundaries Scale",
        description: "Transitional zones between star systems and the interstellar medium are governed by phenomena like stellar winds and the influence of nearby supernovae."
    },
    Scale {
        level: 15,
        order_of_magnitude: 13,
        name: "Stellar Nurseries and Decay Scale",
        description: "Life cycles of stars and early stages of galactic structures are influenced by processes like nuclear fusion, supernova-induced shockwaves, and gravitational collapse."
    },
    Scale {
        level: 14,
        order_of_magnitude: 14,
        name: "Interstellar Matter and Energy Flow Scale",
        description: "This scale covers the interstellar neighborhood, encompassing the flow of matter and energy between star systems. Principles like mass conservation, energy transfer, and the dynamics of the interstellar medium are universally applicable."
    },
    Scale {
        level: 13,
        order_of_magnitude: 15,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 12,
        order_of_magnitude: 16,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 11,
        order_of_magnitude: 17,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 10,
        order_of_magnitude: 18,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 9,
        order_of_magnitude: 19,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 8,
        order_of_magnitude: 20,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 7,
        order_of_magnitude: 21,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 6,
        order_of_magnitude: 22,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 5,
        order_of_magnitude: 23,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 4,
        order_of_magnitude: 24,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 3,
        order_of_magnitude: 25,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 2,
        order_of_magnitude: 26,
        name: "big",
        description: "description_here"
    },
    Scale {
        level: 1,
        order_of_magnitude: 27,
        name: "big",
        description: "description_here"
    },
];

// Constant variables


// Types


// Enums


// Structs
pub struct Scale {
    level: u8,
    order_of_magnitude: i8,
    name: &'static str,
    description: &'static str,
}

// Implementations


// Module Functions
