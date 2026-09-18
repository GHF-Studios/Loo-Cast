use super::*;

#[test]
fn physics_campus_asset_parses_and_validates() {
    let bytes = include_bytes!("../../../assets/maps/physics_campus.spacemap");
    let map: AuthoredMap = ron::de::from_bytes(bytes).expect("campus RON should parse");

    map.validate()
        .expect("campus should satisfy authored-map invariants");
    assert!(map.objects.len() >= 150);
    assert!(map.zones.len() >= 10);
}
