Possible Command/System/Event Input Categories:
- Primitive
- Component
- Resource
- Commands (e.g. BevyCommands, TestCommands, HealthCommands, etc.)
- Query (including filters and other features of bevy queries)

Possible Command Output Categories:
- Primitive
- Component
- Entity

Possible Framework Modules:
- Components
- Resources
- Commands
- Systems
- Events
- Entities
- Archetypes

commands.spawn_entity(
    |error| {
        println!("Error: {}", error);
    },
    |output| {
        commands.despawn_entity(output.entity_id);
    }
);