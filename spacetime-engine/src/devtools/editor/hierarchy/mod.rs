//! ECS hierarchy focus, semantic Structure navigation and USF relationships.

use super::*;

pub(super) fn sync_hierarchy_selection(selected: &mut SelectedEntities, entity: Option<Entity>) {
    let desired = entity.map(|entity| vec![entity]).unwrap_or_default();
    if selected.as_slice() == desired.as_slice() {
        return;
    }

    selected.clear();
    if let Some(entity) = entity {
        selected.select_replace(entity);
    }
}

pub(super) fn apply_hierarchy_selection(world: &mut World, selected: &[Entity]) {
    world.resource_mut::<StructureSelection>().clear();
    let mut focus = world.resource_mut::<DeveloperFocus>();
    focus.clear_pin();

    let [entity] = selected else {
        focus.clear_selection();
        return;
    };
    let entity = *entity;
    drop(focus);

    let target = focus_target_for_entity(world, entity);
    world.resource_mut::<DeveloperFocus>().select(target);
    world
        .resource_mut::<StructureSelection>()
        .select_entity(target);
}

fn focus_target_for_entity(world: &World, entity: Entity) -> FocusTarget {
    let semantic_entity = world
        .get::<UsfManifestationOf>(entity)
        .map(|manifestation| manifestation.0)
        .unwrap_or(entity);
    FocusTarget::entity(entity, semantic_entity)
}

pub(super) fn draw_structure(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select or focus an entity to inspect its semantic structure.");
        return;
    };

    ui.heading(focus_name(Some(target), world));
    ui.weak(format!("ECS {:?}", target.spatial_entity));
    if target.semantic_entity != target.spatial_entity {
        ui.weak(format!("Semantic {:?}", target.semantic_entity));
    }
    ui.add_space(4.0);

    let selected_item = world.resource::<StructureSelection>().item_for(target);
    if ui
        .selectable_label(selected_item.is_none(), "Entity")
        .clicked()
    {
        world.resource_mut::<DeveloperFocus>().select(target);
        world
            .resource_mut::<StructureSelection>()
            .select_entity(target);
    }

    let items = world
        .resource::<StructureFrame>()
        .sorted_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    for item in items {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            let clicked = ui
                .selectable_label(selected_item == Some(item.id), &item.label)
                .clicked();
            if let Some(detail) = &item.detail {
                ui.weak(detail);
            }
            if clicked {
                world.resource_mut::<DeveloperFocus>().select(target);
                world
                    .resource_mut::<StructureSelection>()
                    .select_item(target, item.id);
            }
        });
    }

    let manifestations = world
        .get::<UsfManifestations>(target.semantic_entity)
        .map(|manifestations| manifestations.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    if target.spatial_entity != target.semantic_entity || !manifestations.is_empty() {
        ui.add_space(6.0);
        ui.separator();
        ui.strong("USF relationships");

        if target.spatial_entity != target.semantic_entity
            && ui
                .button(format!("Semantic entity  {:?}", target.semantic_entity))
                .clicked()
        {
            select_related_entity(world, target.semantic_entity);
            return;
        }

        for manifestation in manifestations {
            let name = entity_name(world, manifestation);
            let authority = world
                .get::<UsfManifestationAuthority>(manifestation)
                .is_some();
            let suffix = if authority { "  [authority]" } else { "" };
            if ui
                .button(format!("Manifestation  {name}  {manifestation:?}{suffix}"))
                .clicked()
            {
                select_related_entity(world, manifestation);
                return;
            }
        }
    }
}

fn select_related_entity(world: &mut World, entity: Entity) {
    let target = focus_target_for_entity(world, entity);
    world.resource_mut::<DeveloperFocus>().clear_pin();
    world.resource_mut::<DeveloperFocus>().select(target);
    world
        .resource_mut::<StructureSelection>()
        .select_entity(target);
}

pub(super) fn focus_name(target: Option<FocusTarget>, world: &World) -> String {
    let Some(target) = target else {
        return "No semantic focus".to_owned();
    };

    world
        .get::<Name>(target.semantic_entity)
        .or_else(|| world.get::<Name>(target.spatial_entity))
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:?}", target.semantic_entity))
}

fn entity_name(world: &World, entity: Entity) -> String {
    world
        .get::<Name>(entity)
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| "Unnamed".to_owned())
}
