//! Transform semantic-structure and inspection adapter.

use super::*;

pub(super) fn collect_transform_structure(
    focus: Res<DeveloperFocus>,
    transforms: Query<(), With<Transform>>,
    mut frame: ResMut<StructureFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    if !transforms.contains(target.spatial_entity) {
        return;
    }

    frame.submit(
        StructureItem::new(TRANSFORM_STRUCTURE, "Transform", 10)
            .detail("position, orientation and scale"),
    );
}

pub(super) fn collect_transform_inspection(
    focus: Res<DeveloperFocus>,
    transforms: Query<&Transform>,
    writable: Query<(), With<EditorTransformWritable>>,
    parented: Query<(), With<ChildOf>>,
    mut frame: ResMut<InspectionFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };
    let Ok(transform) = transforms.get(target.spatial_entity) else {
        return;
    };

    let access =
        if writable.contains(target.spatial_entity) && !parented.contains(target.spatial_entity) {
            InspectAccess::Direct
        } else {
            InspectAccess::ReadOnly
        };
    let (rx, ry, rz) = transform.rotation.to_euler(EulerRot::XYZ);

    let translation = InspectField::new("Translation", InspectValue::Vec3(transform.translation))
        .number_input(InspectNumberInput::speed(0.02));
    let rotation = InspectField::new(
        "Rotation (degrees)",
        InspectValue::Vec3(Vec3::new(rx.to_degrees(), ry.to_degrees(), rz.to_degrees())),
    )
    .number_input(InspectNumberInput::speed(0.2));
    let scale = InspectField::new("Scale", InspectValue::Vec3(transform.scale))
        .number_input(InspectNumberInput::speed(0.01));
    let (translation, rotation, scale) = if access.editable() {
        (
            translation.editable(TRANSLATION_FIELD, access),
            rotation.editable(ROTATION_FIELD, access),
            scale.editable(SCALE_FIELD, access),
        )
    } else {
        (translation, rotation, scale)
    };

    frame.submit(
        InspectSection::new(TRANSFORM_SECTION, "Transform", 10)
            .for_structure(TRANSFORM_STRUCTURE)
            .contextual_gizmo()
            .field(translation)
            .field(rotation)
            .field(scale),
    );
}

pub(super) fn apply_transform_inspection_edits(
    mut requests: MessageReader<InspectEditRequest>,
    parented: Query<(), With<ChildOf>>,
    mut transforms: Query<&mut Transform, With<EditorTransformWritable>>,
) {
    for request in requests.read() {
        if request.section != TRANSFORM_SECTION || parented.contains(request.target.spatial_entity)
        {
            continue;
        }
        let Ok(mut transform) = transforms.get_mut(request.target.spatial_entity) else {
            continue;
        };
        let InspectValue::Vec3(value) = request.value else {
            continue;
        };

        match request.field {
            TRANSLATION_FIELD => transform.translation = value,
            ROTATION_FIELD => {
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    value.x.to_radians(),
                    value.y.to_radians(),
                    value.z.to_radians(),
                );
            }
            SCALE_FIELD => transform.scale = value,
            _ => {}
        }
    }
}
