use crate::bevy::prelude::*;
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhenomenonCapability {
    ManifestationDensityField,
    ManifestationMaterialProfile,
}
impl PhenomenonCapability {
    pub fn canonical_id(self) -> &'static str {
        match self {
            Self::ManifestationDensityField => "manifestation_density_field",
            Self::ManifestationMaterialProfile => "manifestation_material_profile",
        }
    }

    pub fn try_from_config_value(raw: &str) -> Result<Self, String> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "manifestation_density_field" | "manifestation-density-field" | "density_field" | "density-field" => Ok(Self::ManifestationDensityField),
            "manifestation_material_profile" | "manifestation-material-profile" | "material_profile" | "material-profile" => {
                Ok(Self::ManifestationMaterialProfile)
            }
            unknown => Err(format!("unknown capability '{unknown}'")),
        }
    }

    pub fn from_config_value(raw: &str) -> Self {
        Self::try_from_config_value(raw).unwrap_or_else(|error| panic!("USF phenomenon capability parse failed: {error}"))
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PhenomenonKind {
    #[default]
    ManifestationDensityDebug,
}

impl PhenomenonKind {
    pub fn canonical_id(self) -> &'static str {
        match self {
            Self::ManifestationDensityDebug => "manifestation_density_debug",
        }
    }

    pub fn try_from_config_value(raw: &str) -> Result<Self, String> {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "manifestation_density_debug" | "manifestation-density-debug" => Ok(Self::ManifestationDensityDebug),
            _ => Err(format!("unknown kind '{}'", normalized)),
        }
    }

    pub fn from_config_value(raw: &str) -> Self {
        Self::try_from_config_value(raw).unwrap_or_else(|error| panic!("USF phenomenon kind parse failed: {}", error))
    }

    pub fn declared_capabilities(self) -> &'static [PhenomenonCapability] {
        match self {
            Self::ManifestationDensityDebug => &[PhenomenonCapability::ManifestationDensityField],
        }
    }

    pub fn supports_capability(self, capability: PhenomenonCapability) -> bool {
        self.declared_capabilities().contains(&capability)
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub struct ManifestationDensityFieldDefinition {
    pub coarse_span_units: f64,
    pub detail_span_units: f64,
    pub coarse_weight: f32,
    pub detail_weight: f32,
    pub bias: f32,
    pub gain: f32,
    pub center: f32,
    pub seed_salt_coarse: u64,
    pub seed_salt_detail: u64,
}
impl Default for ManifestationDensityFieldDefinition {
    fn default() -> Self {
        Self {
            coarse_span_units: 320.0,
            detail_span_units: 128.0,
            coarse_weight: 0.82,
            detail_weight: 0.18,
            bias: 0.66,
            gain: 3.0,
            center: 0.5,
            seed_salt_coarse: 0xa5a5_35f4_9be3_c211_u64,
            seed_salt_detail: 0x8b8b_4fb7_0a7f_6611_u64,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub enum PhenomenonManifestationFieldContract {
    DensityField(ManifestationDensityFieldDefinition),
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub struct ManifestationMaterialProfileDefinition {
    pub albedo_r: f32,
    pub albedo_g: f32,
    pub albedo_b: f32,
    pub alpha: f32,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub emissive_strength: f32,
}
impl Default for ManifestationMaterialProfileDefinition {
    fn default() -> Self {
        Self {
            albedo_r: 0.54,
            albedo_g: 0.68,
            albedo_b: 0.93,
            alpha: 1.0,
            perceptual_roughness: 0.9,
            metallic: 0.0,
            emissive_strength: 0.0,
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

    #[test]
    fn phenomenon_kind_parsing_normalizes_aliases() {
        let underscore = PhenomenonKind::try_from_config_value("manifestation_density_debug").expect("underscore kind id should parse");
        let kebab = PhenomenonKind::try_from_config_value("manifestation-density-debug").expect("kebab kind id should parse");
        assert_eq!(underscore, kebab);
        assert_eq!(underscore.canonical_id(), "manifestation_density_debug");
    }

    #[test]
    fn phenomenon_kind_capability_contract_is_explicit() {
        let kind = PhenomenonKind::ManifestationDensityDebug;
        assert!(kind.supports_capability(PhenomenonCapability::ManifestationDensityField));
    }

    #[test]
    fn phenomenon_capability_parsing_normalizes_aliases() {
        let underscore = PhenomenonCapability::try_from_config_value("manifestation_density_field").expect("underscore capability should parse");
        let kebab = PhenomenonCapability::try_from_config_value("manifestation-density-field").expect("kebab capability should parse");
        assert_eq!(underscore, kebab);
        assert_eq!(underscore.canonical_id(), "manifestation_density_field");

        let material_underscore =
            PhenomenonCapability::try_from_config_value("manifestation_material_profile").expect("material underscore capability should parse");
        let material_alias = PhenomenonCapability::try_from_config_value("material-profile").expect("material alias capability should parse");
        assert_eq!(material_underscore, material_alias);
        assert_eq!(material_underscore.canonical_id(), "manifestation_material_profile");
    }
}
