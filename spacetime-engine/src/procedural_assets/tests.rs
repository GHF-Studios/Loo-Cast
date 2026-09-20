use super::*;

#[test]
fn regenerated_texture_set_keeps_expected_pixel_storage() {
    let recipe = CrackedClayRecipe {
        resolution: UVec2::new(32, 24),
        ..default()
    };
    let generated = generate_cracked_clay(&recipe);
    let expected = (32 * 24 * 4) as usize;

    for image in [
        &generated.albedo,
        &generated.normal,
        &generated.height,
        &generated.orm,
    ] {
        assert_eq!(image.data.as_ref().unwrap().len(), expected);
    }
}

