    use super::*;

    fn query(local: Vec3) -> VoxelQueryPosition {
        VoxelQueryPosition::from_scale0_local(local).unwrap()
    }

    #[test]
    fn procedural_terrain_preserves_existing_near_origin_field() {
        let terrain = ProceduralTerrain::new(42);
        let origin = query(Vec3::ZERO);
        let point = query(Vec3::new(123.5, 0.0, -87.25));

        assert_eq!(
            terrain.height_at(origin, point),
            terrain.height(123.5, -87.25)
        );
        assert_eq!(
            terrain.sample_at(origin, point),
            terrain.sample_at(origin, point)
        );
        assert_ne!(
            terrain.height_at(origin, point),
            ProceduralTerrain::new(43).height_at(origin, point)
        );
    }

    #[test]
    fn procedural_volume_structure_is_genuinely_three_dimensional() {
        let origin = query(Vec3::ZERO);
        let a = query(Vec3::new(17.0, -8.0, 23.0));
        let b = query(Vec3::new(17.0, 11.0, 23.0));
        let av = volumetric_noise(origin, a, 0.041, 77);
        let bv = volumetric_noise(origin, b, 0.041, 77);
        assert_ne!(av, bv);
    }

    #[test]
    fn child_scale_refines_parent_instead_of_regenerating_it() {
        let root_seed = 0x1234_5678_9ABC_DEF0_u64;
        let child_seed = 0x0FED_CBA9_8765_4321_u64;
        let parent_scale = SpatialScale::MAX;
        let child_scale = SpatialScale::new(SPATIAL_SCALE_MAX - 1).unwrap();

        let parent = ProceduralVolume::scale_refinement(
            0x10_0CA57_5EED_2026,
            parent_scale,
            &[(parent_scale, root_seed)],
        );
        let child = ProceduralVolume::scale_refinement(
            0x10_0CA57_5EED_2026,
            child_scale,
            &[(child_scale, child_seed), (parent_scale, root_seed)],
        );

        let parent_point = Vec3::new(7.25, -1.5, -3.75);
        let child_point = parent_point * 10.0;
        let parent_distance = parent.sample_local(parent_point).distance.0;
        let child_distance_in_parent_units = child.sample_local(child_point).distance.0 / 10.0;

        assert!(
            (parent_distance - child_distance_in_parent_units).abs() < 0.08,
            "parent={parent_distance}, child-as-parent={child_distance_in_parent_units}"
        );
    }

    #[test]
    fn sphere_base_is_reconstructible_without_stored_voxels() {
        let origin = query(Vec3::ZERO);
        let base = VoxelBase::sphere(origin, 2.0, VoxelMaterialId::ROCK);
        assert!(base.sample_in_world(origin, origin).distance.is_solid());
        assert!(
            base.sample_in_world(origin, query(Vec3::splat(4.0)))
                .distance
                .is_empty()
        );
    }

    #[test]
    fn canonical_fallback_noise_is_continuous_across_a_usf_digit_carry() {
        let left = query(Vec3::new(499.75, 0.0, 0.0));
        let right = left.translated(Vec3::new(0.5, 0.0, 0.0)).unwrap();
        let left_value = semantic_value_noise(left, 25, 42);
        let right_value = semantic_value_noise(right, 25, 42);

        assert!((left_value - right_value).abs() < 0.25);
    }
