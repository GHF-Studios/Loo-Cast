use bevy::prelude::*;
use bevy::ecs::component::{ComponentId, ComponentInfo};

pub fn get_components_ids<'a>(world: &'a World, entity: &Entity) -> impl Iterator<Item=ComponentId> + 'a
{
    for archetype in world.archetypes().iter()
    {
        for archetype_entity in archetype.entities() {
            if archetype_entity.id() == *entity {
                return archetype.components();
            }
        }
    }

    panic!("Entity not found in any archetype!");
}

pub fn component_id_to_component_info(world: &World, component_id: ComponentId) -> Option<&ComponentInfo>
{
    let components = world.components();
    components.get_info(component_id)
}

pub fn extract_component_name(component_info: &ComponentInfo) -> &str
{
    component_info.name()
}