//! Compile-visible proof that the selected transition-cell backend can extract
//! ordinary and transition-face geometry from the same scalar field.
//!
//! This is intentionally not a gameplay authority path yet.

use transvoxel::prelude::*;

fn sphere_density(x: f32, y: f32, z: f32) -> f32 {
    1.0 - (x * x + y * y + z * z).sqrt() / 5.0
}

#[test]
fn transition_cell_backend_extracts_a_balanced_boundary() {
    let block = Block::new([0.0, 0.0, 0.0], 10.0, 10);
    let regular = extract_from_field(
        &sphere_density,
        FieldCaching::CacheNothing,
        block,
        TransitionSide::none(),
        0.0,
        GenericMeshBuilder::new(),
    )
    .build();

    let transitioned = extract_from_field(
        &sphere_density,
        FieldCaching::CacheNothing,
        block,
        TransitionSide::LowX.into(),
        0.0,
        GenericMeshBuilder::new(),
    )
    .build();

    assert!(!regular.tris().is_empty());
    assert!(!transitioned.tris().is_empty());
    assert!(
        transitioned.tris().len() > regular.tris().len(),
        "a surface crossing the requested boundary should emit transition topology",
    );
}
