use crate::bevy::prelude::*;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PhenomenonKind {
    #[default]
    Mandelbulb,
    SierpinskiSponge,
}

impl PhenomenonKind {
    pub fn from_config_value(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "sierpinski_sponge" | "sierpinski-sponge" | "sierpinski" | "sponge" | "menger" => Self::SierpinskiSponge,
            _ => Self::Mandelbulb,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhenomenonId(pub u64);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhenomenonNodeSeed(pub u64);

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhenomenonNodeKey {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub cell3: IVec3,
    pub parent: Option<PhenomenonNodeSeed>,
    pub local_index: u32,
}

impl PhenomenonNodeKey {
    pub fn deterministic_seed(self) -> PhenomenonNodeSeed {
        let parent = self.parent.unwrap_or(PhenomenonNodeSeed(0x2f77_9b97_f4a7_c15d));
        let mut state = mix64(0xc6a4_a793_5bd1_e995_u64 ^ self.phenomenon_id.0);
        state = mix64(state ^ self.scale.index_from_top() as u64);
        state = mix64(state ^ fold_signed(self.cell3.x));
        state = mix64(state ^ fold_signed(self.cell3.y));
        state = mix64(state ^ fold_signed(self.cell3.z));
        state = mix64(state ^ parent.0);
        state = mix64(state ^ self.local_index as u64);
        if state == 0 {
            return PhenomenonNodeSeed(0x9e37_79b9_7f4a_7c15);
        }
        PhenomenonNodeSeed(state)
    }
}

fn fold_signed(value: i32) -> u64 {
    value as i64 as u64
}

fn mix64(mut state: u64) -> u64 {
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phenomenon_node_seed_is_deterministic_for_same_key() {
        let key = PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(7),
            scale: Scale::ScaleMeter1,
            cell3: IVec3::new(-2, 1, 4),
            parent: Some(PhenomenonNodeSeed(42)),
            local_index: 9,
        };
        let first = key.deterministic_seed();
        let second = key.deterministic_seed();
        assert_eq!(first, second);
    }

    #[test]
    fn phenomenon_node_seed_changes_when_lineage_changes() {
        let base = PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(7),
            scale: Scale::ScaleMeter1,
            cell3: IVec3::new(1, 2, 3),
            parent: Some(PhenomenonNodeSeed(5)),
            local_index: 0,
        };
        let sibling = PhenomenonNodeKey {
            local_index: 1,
            ..base
        };
        assert_ne!(base.deterministic_seed(), sibling.deterministic_seed());
    }
}
