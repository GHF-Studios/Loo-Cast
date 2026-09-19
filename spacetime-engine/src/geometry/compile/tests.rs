use super::*;

#[test]
fn staircase_generates_one_solid_column_per_tread() {
    let definition = StaircaseDef {
        id: "stairs".into(),
        zone: "test".into(),
        origin: (0.0, 0.0, 0.0),
        yaw_degrees: 0.0,
        steps: 3,
        width: 100.0,
        step_height: 10.0,
        step_depth: 20.0,
        material: "concrete".into(),
        tags: Vec::new(),
    };

    let mut output = Vec::new();
    compile_staircase(&mut output, &definition);

    assert_eq!(output.len(), 3);
    let CompiledNode::Geometry(third) = &output[2] else {
        panic!("expected geometry");
    };
    let CompiledShape::Box { size } = &third.shape else {
        panic!("expected box");
    };
    assert_eq!(size.y, 30.0);
    assert_eq!(third.transform.translation, Vec3::new(0.0, 15.0, 50.0));
}

#[test]
fn ramp_top_surface_starts_at_authored_start() {
    let definition = RampDef {
        id: "ramp".into(),
        zone: "test".into(),
        start: (0.0, 0.0, 0.0),
        yaw_degrees: 0.0,
        width: 100.0,
        run: 100.0,
        rise: 100.0,
        thickness: 10.0,
        material: "concrete".into(),
        solid: true,
        tags: Vec::new(),
    };

    let ramp = compile_ramp(&definition);
    let local_up = ramp.transform.rotation * Vec3::Y;
    let CompiledShape::Box { size } = ramp.shape else {
        panic!("expected box");
    };
    let top_surface_center = ramp.transform.translation + local_up * size.y * 0.5;

    assert!((top_surface_center - Vec3::new(0.0, 50.0, 50.0)).length() < 0.0001);
}
