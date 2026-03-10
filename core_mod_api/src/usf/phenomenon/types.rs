use crate::bevy::prelude::*;
use crate::usf::pos::types::LocalCell3;
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

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PhenomenonLineage {
    pub cells: Vec<LocalCell3>,
}
impl PhenomenonLineage {
    pub fn root() -> Self {
        Self { cells: vec![LocalCell3::ZERO] }
    }

    pub fn from_cells(cells: Vec<LocalCell3>) -> Self {
        Self { cells }
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn depth(&self) -> u32 {
        self.cells.len().saturating_sub(1) as u32
    }

    pub fn leaf(&self) -> Option<LocalCell3> {
        self.cells.last().copied()
    }

    pub fn pushed(&self, next: LocalCell3) -> Self {
        let mut cells = self.cells.clone();
        cells.push(next);
        Self { cells }
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhenomenonNodeKey {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub lineage: PhenomenonLineage,
    pub parent: Option<PhenomenonNodeSeed>,
    pub local_index: u32,
}

impl PhenomenonNodeKey {
    pub fn local_cell(&self) -> LocalCell3 {
        self.lineage.leaf().unwrap_or(LocalCell3::ZERO)
    }

    pub fn deterministic_seed(&self) -> PhenomenonNodeSeed {
        let parent = self.parent.unwrap_or(PhenomenonNodeSeed(0x2f77_9b97_f4a7_c15d));
        let mut state = mix64(0xc6a4_a793_5bd1_e995_u64 ^ self.phenomenon_id.0);
        state = mix64(state ^ self.scale.index_from_top() as u64);
        state = mix64(state ^ self.lineage.cells.len() as u64);
        for local_cell in &self.lineage.cells {
            let cell = local_cell.as_ivec3();
            state = mix64(state ^ fold_signed(cell.x));
            state = mix64(state ^ fold_signed(cell.y));
            state = mix64(state ^ fold_signed(cell.z));
        }
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
            lineage: PhenomenonLineage::from_cells(vec![LocalCell3::new_local(0, 0, 0), LocalCell3::new_local(-2, 1, 4)]),
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
            lineage: PhenomenonLineage::from_cells(vec![LocalCell3::new_local(0, 0, 0), LocalCell3::new_local(1, 2, 3)]),
            parent: Some(PhenomenonNodeSeed(5)),
            local_index: 0,
        };
        let sibling = PhenomenonNodeKey {
            local_index: 1,
            ..base.clone()
        };
        assert_ne!(base.deterministic_seed(), sibling.deterministic_seed());
    }

    #[test]
    fn phenomenon_node_seed_changes_when_full_lineage_changes() {
        let a = PhenomenonNodeKey {
            phenomenon_id: PhenomenonId(11),
            scale: Scale::ScaleMeter1,
            lineage: PhenomenonLineage::from_cells(vec![LocalCell3::new_local(0, 0, 0), LocalCell3::new_local(1, 0, 0)]),
            parent: Some(PhenomenonNodeSeed(999)),
            local_index: 0,
        };
        let b = PhenomenonNodeKey {
            lineage: PhenomenonLineage::from_cells(vec![LocalCell3::new_local(0, 0, 0), LocalCell3::new_local(2, 0, 0)]),
            ..a.clone()
        };
        assert_ne!(a.deterministic_seed(), b.deterministic_seed());
    }
}
