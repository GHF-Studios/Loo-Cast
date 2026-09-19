use super::*;

#[test]
fn box_mesh_contains_twelve_triangles() {
    let mesh = full_box_mesh(SpatialSplitBox::from_size(Vec3::splat(2.0)));
    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    assert_eq!(positions.len(), 36);
}
