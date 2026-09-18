use super::*;

#[test]
fn defaults_are_valid() {
    EngineConfig::default().validate().unwrap();
}

#[test]
fn partial_ron_inherits_structured_defaults() {
    let config: EngineConfig = ron::from_str(
        r#"(
            voxel: (
                manifestation: (
                    grouping: (
                        base_chunks_per_axis: 20,
                    ),
                ),
            ),
        )"#,
    )
    .unwrap();

    assert_eq!(config.voxel.manifestation.grouping.base_chunks_per_axis, 20);
    assert_eq!(config.voxel.manifestation.rebuild_budget_per_frame, 8);
    assert_eq!(config.voxel.streaming.default_load_budget_per_frame, 24);
    config.validate().unwrap();
}

#[test]
fn invalid_alignment_is_rejected() {
    let mut config = EngineConfig::default();
    config.voxel.manifestation.grouping.base_chunks_per_axis = 3;
    assert!(config.validate().is_err());
}

#[test]
fn runtime_overrides_are_highest_priority() {
    let mut config = EngineConfig::default();
    let mut overrides = EngineConfigOverrides::default();
    overrides.voxel.manifestation.grouping.base_chunks_per_axis = Some(20);

    config.apply_overrides(&overrides);
    assert_eq!(config.voxel.manifestation.grouping.base_chunks_per_axis, 20);
}
