use bevy::ecs::entity::Entity as Entity;
use bevy::ecs::query::QueryData as QueryData;
use bevy::ecs::query::QueryFilter as QueryFilter;
use bevy::ecs::bundle::Bundle as Bundle;
use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;

private fn run_ecs_world_resource_component_schedule_test(world) {
    print("[Startup][ECS] world/resource/component/schedule test");
    print("[Startup][ECS] registered component ctors = " + bevy::ecs::component::registered_ids().len());
    print("[Startup][ECS] registered resource types = " + bevy::ecs::resource::registered_type_ids().len());
    print("[Startup][ECS] registered message types = " + bevy::ecs::message::registered_type_ids().len());

    let resource_type_id = bevy::ecs::resource::script_probe_type_id();
    world.init_resource(resource_type_id);
    world.insert_resource(resource_type_id, "probe-resource-0");
    let resource_value = world.get_resource(resource_type_id);
    print("[Startup][ECS] resource get = " + resource_value.to_string());

    if !world.has_resource(resource_type_id) {
        print("[Startup][ECS][WARN] expected has_resource(resource_type_id) == true");
    }

    let resource_mut_value = world.get_resource_mut(resource_type_id);
    print("[Startup][ECS] resource get_mut = " + resource_mut_value.to_string());

    let removed_resource = world.remove_resource(resource_type_id);
    print("[Startup][ECS] resource removed = " + removed_resource.to_string());

    world.insert_resource(resource_type_id, "probe-resource-1");

    let message_type_id = bevy::ecs::message::script_probe_type_id();
    world.write_message(message_type_id, "typed-message-0");
    world.write_message(message_type_id, "typed-message-1");
    let drained_messages = world.drain_messages(message_type_id);
    print("[Startup][ECS] typed drain remaining_len = " + drained_messages.remaining_len());

    let entity_data = QueryData::single_t(Entity);
    if !world.exists(entity_data) {
        print("[Startup][ECS][WARN] expected world.exists(QueryData::single(Entity)) == true");
    }

    let spawned_ids = world.spawn_batch([
        PlayerBundle::as_trait_obj(PlayerBundle::new_default(), Bundle),
        PlayerBundle::as_trait_obj(PlayerBundle::new_default(), Bundle),
    ]);
    print("[Startup][ECS] spawn_batch count = " + spawned_ids.len());

    let spawned_from_components = world.spawn_components([
        bevy::ecs::component::create_unit(bevy::ecs::component::chunk_actor_type_id()),
    ]);
    print("[Startup][ECS] spawn_components entity = " + spawned_from_components.to_string());

    world.commands(|commands| {
        let deferred_spawned = commands.spawn_components([
            bevy::ecs::component::create_unit(bevy::ecs::component::chunk_loader_type_id()),
        ]);
        print("[Startup][ECS] commands.spawn_components entity = " + deferred_spawned.to_string());

        let deferred_batch = commands.spawn_components_batch([
            [bevy::ecs::component::create_unit(bevy::ecs::component::chunk_actor_type_id())],
            [bevy::ecs::component::create_unit(bevy::ecs::component::chunk_loader_type_id())],
        ]);
        print("[Startup][ECS] commands.spawn_components_batch count = " + deferred_batch.len());

        commands.entity(deferred_spawned, |entity_commands| {
            entity_commands.insert_components([
                bevy::ecs::component::create_unit(bevy::ecs::component::chunk_actor_type_id()),
            ]);
            entity_commands.remove_component(bevy::ecs::component::chunk_actor_type_id());
        });
    });

    world.spawn_empty(|entity_world_mut| {
        entity_world_mut.insert_components([
            bevy::ecs::component::create_unit(bevy::ecs::component::chunk_actor_type_id()),
            bevy::ecs::component::create_unit(bevy::ecs::component::chunk_loader_type_id()),
        ]);
    });

    let chunk_actor_filter = QueryFilter::require(bevy::ecs::component::chunk_actor_type_id());
    let chunk_loader_filter = QueryFilter::require(bevy::ecs::component::chunk_loader_type_id());
    print(
        "[Startup][ECS] exists_filtered ChunkActor = " +
        world.exists_filtered(entity_data, chunk_actor_filter)
    );
    print(
        "[Startup][ECS] exists_filtered ChunkLoader = " +
        world.exists_filtered(entity_data, chunk_loader_filter)
    );

    let first_player = world.single_filtered(entity_data, QueryFilter::require(bevy::ecs::component::player_type_id()));
    if first_player != () {
        world.get_entity(first_player, |entity_ref| {
            print("[Startup][ECS] get_entity id = " + entity_ref.id().to_string());
        });

        world.get_entity_mut(first_player, |entity_world_mut| {
            entity_world_mut.insert_component(bevy::ecs::component::create_unit(bevy::ecs::component::chunk_actor_type_id()));
            entity_world_mut.remove_component(bevy::ecs::component::chunk_actor_type_id());
        });
    }

    print("[Startup][ECS] known_schedules count = " + world.known_schedules().len());

    if world.run_schedule("definitely_not_a_schedule") {
        print("[Startup][ECS][WARN] expected run_schedule on unknown schedule to return false");
    }
}
