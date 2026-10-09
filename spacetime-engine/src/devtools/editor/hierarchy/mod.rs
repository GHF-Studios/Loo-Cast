//! ECS hierarchy focus, semantic Structure navigation and USF relationships.

use super::*;

/// Editor-only organization. This relationship has no Transform parentage,
/// simulation ownership, constituency, or linked-spawn lifetime semantics.
#[derive(Component, Debug)]
#[relationship(relationship_target = EditorGroupMembers)]
pub struct EditorGroupOf(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = EditorGroupOf)]
pub struct EditorGroupMembers(Vec<Entity>);

#[derive(Component, Debug)]
pub(super) struct EditorGroup;

pub(super) fn draw_editor_groups(
    ui: &mut egui::Ui,
    world: &mut World,
    selected: &mut SelectedEntities,
) {
    ui.horizontal(|ui| {
        ui.strong("Editor groups");
        if ui.small_button("+ Group").clicked() {
            let group = world.spawn((Name::new("New Group"), EditorGroup)).id();
            selected.select_replace(group);
        }
    });

    let groups = world
        .query_filtered::<Entity, With<EditorGroup>>()
        .iter(world)
        .collect::<Vec<_>>();
    if let [entity] = selected.as_slice() {
        let entity = *entity;
        let mut target = None;
        ui.menu_button("Move selected entity into…", |ui| {
            for &group in &groups {
                if !can_group(world, entity, group) {
                    continue;
                }
                if ui.button(entity_name(world, group)).clicked() {
                    target = Some(group);
                    ui.close();
                }
            }
        });
        if let Some(group) = target {
            world.entity_mut(entity).insert(EditorGroupOf(group));
        }
        if world.get::<EditorGroupOf>(entity).is_some()
            && ui
                .small_button("Remove selected entity from group")
                .clicked()
        {
            world.entity_mut(entity).remove::<EditorGroupOf>();
        }
    }

    for group in groups {
        if world.get::<EditorGroupOf>(group).is_none() {
            draw_group_node(ui, world, selected, group, 0);
        }
    }
    ui.separator();
}

fn can_group(world: &World, entity: Entity, group: Entity) -> bool {
    if entity == group || !world.entities().contains(entity) {
        return false;
    }
    let mut cursor = Some(group);
    for _ in 0..64 {
        let Some(current) = cursor else {
            return true;
        };
        if current == entity {
            return false;
        }
        cursor = world.get::<EditorGroupOf>(current).map(|parent| parent.0);
    }
    false
}

fn draw_group_node(
    ui: &mut egui::Ui,
    world: &World,
    selected: &mut SelectedEntities,
    group: Entity,
    depth: usize,
) {
    if depth >= 32 {
        ui.weak("Group nesting limit reached");
        return;
    }
    let children = world
        .get::<EditorGroupMembers>(group)
        .map(|members| members.0.clone())
        .unwrap_or_default();
    egui::CollapsingHeader::new(entity_name(world, group))
        .id_salt(group)
        .show(ui, |ui| {
            if ui
                .selectable_label(selected.as_slice() == [group], "Select group")
                .clicked()
            {
                selected.select_replace(group);
            }
            for child in children {
                if world.get::<EditorGroup>(child).is_some() {
                    draw_group_node(ui, world, selected, child, depth + 1);
                } else if ui
                    .selectable_label(selected.as_slice() == [child], entity_name(world, child))
                    .clicked()
                {
                    selected.select_replace(child);
                }
            }
        });
}

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
    let semantic_entity = runtime_semantic_of_world(world, entity).unwrap_or(entity);
    FocusTarget::entity(entity, semantic_entity)
}

pub(super) fn draw_structure(ui: &mut egui::Ui, world: &mut World) {
    let Some(target) = world.resource::<DeveloperFocus>().current() else {
        ui.weak("Select or focus an entity to inspect its semantic structure.");
        return;
    };

    draw_focus_header(ui, world, target);
    draw_structure_items(ui, world, target);
    draw_usf_relationships(ui, world, target);
}

fn draw_focus_header(ui: &mut egui::Ui, world: &World, target: FocusTarget) {
    ui.heading(focus_name(Some(target), world));
    ui.weak(format!("ECS {:?}", target.spatial_entity));
    if target.semantic_entity != target.spatial_entity {
        ui.weak(format!("Semantic {:?}", target.semantic_entity));
    }
    ui.add_space(4.0);
}

fn draw_structure_items(ui: &mut egui::Ui, world: &mut World, target: FocusTarget) {
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
}

fn draw_usf_relationships(ui: &mut egui::Ui, world: &mut World, target: FocusTarget) {
    let partitions = world
        .get::<UsfAuthorityPartitions>(target.semantic_entity)
        .map(|partitions| partitions.iter().collect::<Vec<_>>())
        .unwrap_or_default();
    if target.spatial_entity == target.semantic_entity && partitions.is_empty() {
        return;
    }
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
    if let Some(peer) = world.get::<SpatialSplitPeer>(target.spatial_entity)
        && ui
            .button(format!("Split authority  {:?}", peer.authority))
            .clicked()
    {
        select_related_entity(world, peer.authority);
        return;
    }
    for partition in partitions {
        let partition_name = entity_name(world, partition);
        if ui
            .button(format!(
                "Authority partition  {partition_name}  {partition:?}"
            ))
            .clicked()
        {
            select_related_entity(world, partition);
            return;
        }
        let realizations = world
            .get::<UsfLogicalRealizations>(partition)
            .map(|realizations| realizations.iter().collect::<Vec<_>>())
            .unwrap_or_default();
        for realization in realizations {
            let name = entity_name(world, realization);
            if ui
                .button(format!("Logical realization  {name}  {realization:?}"))
                .clicked()
            {
                select_related_entity(world, realization);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_grouping_rejects_cycles_without_transform_parentage() {
        let mut world = World::new();
        let root = world.spawn((Name::new("Root"), EditorGroup)).id();
        let child = world
            .spawn((Name::new("Child"), EditorGroup, EditorGroupOf(root)))
            .id();
        assert!(!can_group(&world, root, child));
        assert!(can_group(&world, child, root));
        assert!(world.get::<ChildOf>(child).is_none());
    }
}
