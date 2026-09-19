//! Semantic inspector and contextual-gizmo editor panels.

use super::*;

pub(super) fn draw_semantic_inspector(ui: &mut egui::Ui, world: &mut World) {
    let focus = *world.resource::<DeveloperFocus>();
    let Some(target) = focus.current() else {
        ui.weak("No semantic focus.");
        ui.label("Hover the Game view or select an entity in Hierarchy.");
        return;
    };

    let scope = world.resource::<StructureSelection>().item_for(target);
    ui.heading(focus_name(Some(target), world));
    if focus.selected().is_some() {
        ui.weak("selected");
    } else if let Some(hit) = target.hit {
        ui.weak(format!("hover focus · {:.3} m", hit.distance_meters));
    } else {
        ui.weak("focus");
    }
    ui.separator();

    let output = {
        let frame = world.resource::<InspectionFrame>();
        inspect_ui::draw_sections(ui, frame, target, scope, true)
    };
    dispatch_inspection_ui_output(world, output);
}

pub(super) fn draw_gizmos(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select or focus an entity to discover contextual gizmos.");
        return;
    };
    let scope = world.resource::<StructureSelection>().item_for(target);

    if scope == Some(super::super::gizmo::TRANSFORM_STRUCTURE) || scope.is_none() {
        if world.get::<Transform>(target.spatial_entity).is_some() {
            draw_transform_gizmo_panel(ui, world, target);
            return;
        }
    }

    let Some(scope) = scope else {
        ui.weak("Select a Structure item to open its contextual gizmo.");
        return;
    };

    ui.heading("Context gizmo");
    ui.weak("Rich domain context for the selected Structure item. Observation is independent from edit authority.");
    ui.separator();
    let output = {
        let frame = world.resource::<InspectionFrame>();
        inspect_ui::draw_contextual_gizmo(ui, frame, target, scope)
    };
    dispatch_inspection_ui_output(world, output);
}

fn draw_transform_gizmo_panel(ui: &mut egui::Ui, world: &mut World, target: FocusTarget) {
    let entity = target.spatial_entity;
    let Some(transform) = world.get::<Transform>(entity).copied() else {
        ui.weak("No Transform gizmo applies to this focus.");
        return;
    };

    ui.heading("Transform");
    ui.weak(
        "Combined translate + rotate + scale viewport gizmo. Translate/Rotate honor World/Local; Scale remains local to match Transform.scale.",
    );
    ui.horizontal(|ui| {
        ui.label("Space");
        draw_transform_space_control(ui, world);
    });
    ui.separator();

    let parented = world.get::<ChildOf>(entity).is_some();
    let mut writable = world.get::<EditorTransformWritable>(entity).is_some();
    let response = ui.add_enabled(
        !parented,
        egui::Checkbox::new(&mut writable, "Allow direct runtime Transform editing"),
    );
    if response.changed() {
        if writable {
            world.entity_mut(entity).insert(EditorTransformWritable);
        } else {
            world.entity_mut(entity).remove::<EditorTransformWritable>();
        }
    }

    if parented {
        ui.weak(
            "Read-only generic gizmo: parented transforms need a parent-aware/domain authoring adapter before direct mutation is safe.",
        );
    } else if !writable {
        ui.weak("Read-only. Inspectability does not grant mutation authority.");
    } else {
        ui.weak("Runtime-direct authority only; generated or authored sources may still overwrite this value.");
    }

    ui.add_space(6.0);
    let editable = writable && !parented;
    if editable {
        let mut proposed = transform;
        if TransformWidget.edit(ui, &mut proposed, &InspectWidgetContext::new("Transform")) {
            let mut edits = Vec::new();
            if proposed.translation != transform.translation {
                edits.push((
                    super::super::gizmo::TRANSLATION_FIELD,
                    InspectValue::Vec3(proposed.translation),
                ));
            }
            if proposed.rotation != transform.rotation {
                let (rx, ry, rz) = proposed.rotation.to_euler(EulerRot::XYZ);
                edits.push((
                    super::super::gizmo::ROTATION_FIELD,
                    InspectValue::Vec3(Vec3::new(
                        rx.to_degrees(),
                        ry.to_degrees(),
                        rz.to_degrees(),
                    )),
                ));
            }
            if proposed.scale != transform.scale {
                edits.push((
                    super::super::gizmo::SCALE_FIELD,
                    InspectValue::Vec3(proposed.scale),
                ));
            }
            for (field, value) in edits {
                world.write_message(InspectEditRequest {
                    target,
                    section: super::super::gizmo::TRANSFORM_SECTION,
                    field,
                    value,
                });
            }
        }
    } else {
        TransformWidget.show(ui, &transform, &InspectWidgetContext::new("Transform"));
    }

    ui.add_space(6.0);
    ui.separator();
    ui.label(
        "Viewport: translation arrows + rotation rings + scale handles are active simultaneously.",
    );
    if !editable {
        ui.weak("The viewport gizmo remains visible for observation, but dragging is read-only.");
    }
}

fn dispatch_inspection_ui_output(world: &mut World, output: inspect_ui::InspectionUiOutput) {
    for edit in output.edits {
        world.write_message(edit);
    }
    for action in output.actions {
        world.write_message(action);
    }
}
