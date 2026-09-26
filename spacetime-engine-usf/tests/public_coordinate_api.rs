use glam::Vec3;
use spacetime_engine_usf::{UsfCoordinate, UsfPosition, UsfPositionError};

fn assert_coordinate_type(_: UsfCoordinate) {}

#[test]
fn scalar_coordinate_is_public_plain_decimal_algebra() {
    let origin = UsfPosition::default();
    let point = origin
        .translated_whole_native([20_000, -750, 0])
        .unwrap();

    assert_coordinate_type(point.x());
    assert_eq!(point.x().to_string(), "20000");
    assert_eq!(point.y().to_string(), "-750");

    assert_eq!(
        point.relative_native_axis_bounded(&origin, 1, 1_000.0)
            .unwrap(),
        -750.0
    );
    assert!(point.relative_native_axis_is_negative(&origin, 1).unwrap());

    assert_eq!(
        point.relative_native_axis_bounded(&origin, 0, 512.0),
        Err(UsfPositionError::RelativePositionOutsideBound)
    );
    assert!(!point.relative_native_axis_is_negative(&origin, 0).unwrap());

    let nearby = origin
        .translated_native(Vec3::new(0.0, 125.0, 0.0))
        .unwrap();
    assert_eq!(
        nearby.relative_native_axis_bounded(&origin, 1, 256.0)
            .unwrap(),
        125.0
    );
}
