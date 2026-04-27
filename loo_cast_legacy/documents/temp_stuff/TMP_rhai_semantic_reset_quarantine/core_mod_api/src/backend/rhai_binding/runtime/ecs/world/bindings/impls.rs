use rhai::{Array, Dynamic, FnPtr, ImmutableString, NativeCallContext};

use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::bevy::prelude::Commands as BevyCommands;
use crate::bevy::prelude::Entity as BevyEntity;
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;

use crate::rhai_binding::runtime::ecs::{
    bundle::internals::{statics::resolve_bundle_spawn_dispatch, trait_objects::BundleTraitObject},
    component::{bindings::types::Component, internals::statics::COMPONENT_CTOR_REGISTRY},
    resource::internals::statics::{
        resolve_resource_get_dispatch, resolve_resource_get_mut_dispatch, resolve_resource_init_dispatch, resolve_resource_insert_dispatch,
        resolve_resource_remove_dispatch,
    },
    system::commands::bindings::types::Commands,
    system::query::bindings::types::{Query, QueryData, QueryFilter},
    system::query::internals::traits::QueryApi,
    world::{
        bindings::types::World,
        entity_ref::bindings::types::{EntityRef, EntityWorldMut},
        internals::{
            access_requests::{
                DrainMessagesRequest, WORLD_ACCESS_METHOD_DRAIN_MESSAGES, WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES, WORLD_ACCESS_METHOD_ENTITY,
                WORLD_ACCESS_METHOD_ENTITY_MUT, WORLD_ACCESS_METHOD_QUERY, WORLD_ACCESS_METHOD_SPAWN_SINGLE, WORLD_ACCESS_METHOD_WRITE_MESSAGE,
                WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE, WorldEntityRequest, WorldQueryRequest, WorldSpawnSingleRequest, WriteMessageRequest,
                WriteProbeMessageRequest,
            },
            traits::WorldApi,
        },
    },
};
use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter;

const KNOWN_SCHEDULES: &[&str] = &[
    "pre_startup",
    "startup",
    "post_startup",
    "first",
    "pre_update",
    "update",
    "post_update",
    "last",
    "fixed_first",
    "fixed_pre_update",
    "fixed_update",
    "fixed_post_update",
    "fixed_last",
    "update_scale_meter_1",
];

fn run_known_schedule(world: &mut crate::bevy::prelude::World, schedule_name: &str) -> bool {
    match schedule_name {
        "pre_startup" => world.run_schedule(crate::bevy::prelude::PreStartup),
        "startup" => world.run_schedule(crate::bevy::prelude::Startup),
        "post_startup" => world.run_schedule(crate::bevy::prelude::PostStartup),
        "first" => world.run_schedule(crate::bevy::prelude::First),
        "pre_update" => world.run_schedule(crate::bevy::prelude::PreUpdate),
        "update" => world.run_schedule(crate::bevy::prelude::Update),
        "post_update" => world.run_schedule(crate::bevy::prelude::PostUpdate),
        "last" => world.run_schedule(crate::bevy::prelude::Last),
        "fixed_first" => world.run_schedule(crate::bevy::prelude::FixedFirst),
        "fixed_pre_update" => world.run_schedule(crate::bevy::prelude::FixedPreUpdate),
        "fixed_update" => world.run_schedule(crate::bevy::prelude::FixedUpdate),
        "fixed_post_update" => world.run_schedule(crate::bevy::prelude::FixedPostUpdate),
        "fixed_last" => world.run_schedule(crate::bevy::prelude::FixedLast),
        "update_scale_meter_1" => world.run_schedule(crate::core::schedules::UpdateScaleMeter1),
        _ => return false,
    };
    true
}

impl WorldApi for World {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();

        let commands_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyCommands<'static, 'static>,
        > = unsafe { world.start_access("commands", Box::new(())) };
        let commands_binding = Commands {
            commands: commands_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (commands_binding,));
        unsafe { world.end_access(commands_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn flush(&self) {
        let mut world = self.world.start_write();
        world.flush();
        self.world.end_write(world);
    }

    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access("spawn_empty", Box::new(())) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_world_mut,));
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn spawn_single(&self, bundle: BundleTraitObject, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        let spawn_request = WorldSpawnSingleRequest::new(bundle);

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_SPAWN_SINGLE, Box::new(spawn_request)) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_world_mut,));
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn spawn_batch(&self, bundles: Array) -> Array {
        let mut world = self.world.start_write();
        let mut entities = Array::new();

        for (idx, value) in bundles.into_iter().enumerate() {
            let Some(bundle) = value.clone().try_cast::<BundleTraitObject>() else {
                panic!("World::spawn_batch expects BundleTraitObject at index {idx}");
            };

            let dispatch = resolve_bundle_spawn_dispatch(&bundle);
            let mut entity_world_mut = world.spawn_empty();
            dispatch(&mut entity_world_mut, bundle);
            entities.push(Dynamic::from(entity_world_mut.id()));
        }

        self.world.end_write(world);
        entities
    }

    fn spawn_components(&self, components: Array) -> BevyEntity {
        let mut world = self.world.start_write();
        let mut entity_world_mut = world.spawn_empty();

        for (idx, value) in components.into_iter().enumerate() {
            let Some(component) = value.clone().try_cast::<Component>() else {
                panic!("World::spawn_components expects Component at index {idx}");
            };
            let (component_id, params) = component.0;
            let ctor = COMPONENT_CTOR_REGISTRY()
                .get(component_id.as_ref())
                .copied()
                .unwrap_or_else(|| panic!("Component ctor '{}' is not registered", component_id));
            ctor(&mut entity_world_mut, params);
        }

        let entity_id = entity_world_mut.id();
        self.world.end_write(world);
        entity_id
    }

    fn entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        let entity_request = WorldEntityRequest::new(entity);

        let entity_ref_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            crate::bevy::prelude::EntityRef<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_ENTITY, Box::new(entity_request)) };
        let entity_ref = EntityRef {
            entity_ref: entity_ref_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_ref,));
        unsafe { world.end_access(entity_ref_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn entity_mut(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        let entity_request = WorldEntityRequest::new(entity);

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_ENTITY_MUT, Box::new(entity_request)) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_world_mut,));
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn get_entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        if world.get_entity(entity).is_err() {
            self.world.end_write(world);
            return Dynamic::UNIT;
        }

        let entity_request = WorldEntityRequest::new(entity);
        let entity_ref_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            crate::bevy::prelude::EntityRef<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_ENTITY, Box::new(entity_request)) };
        let entity_ref = EntityRef {
            entity_ref: entity_ref_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_ref,));
        unsafe { world.end_access(entity_ref_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn get_entity_mut(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        if world.get_entity(entity).is_err() {
            self.world.end_write(world);
            return Dynamic::UNIT;
        }

        let entity_request = WorldEntityRequest::new(entity);
        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_ENTITY_MUT, Box::new(entity_request)) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_world_mut,));
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn despawn(&self, entity: BevyEntity) -> bool {
        let mut world = self.world.start_write();
        let removed = world.despawn(entity);
        self.world.end_write(world);
        removed
    }

    fn entities(&self) -> Array {
        let mut query = self.query(QueryData::single("bevy::ecs::entity::Entity".into()));
        query.collect_remaining()
    }

    fn query(&self, data: QueryData) -> Query {
        self.query_filtered(data, QueryFilter::none())
    }

    fn query_filtered(&self, data: QueryData, filter: QueryFilter) -> Query {
        let mut world = self.world.start_write();
        let query_request = WorldQueryRequest::new(data, filter);

        let query_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<crate::rhai_binding::value_semantics::access_cell::Scoped, Query> =
            unsafe { world.start_access(WORLD_ACCESS_METHOD_QUERY, Box::new(query_request)) };
        let query = query_raw_handle.take();
        unsafe { world.end_access(query_raw_handle) };

        self.world.end_write(world);

        query
    }

    fn single(&self, data: QueryData) -> Dynamic {
        self.single_filtered(data, QueryFilter::none())
    }

    fn single_filtered(&self, data: QueryData, filter: QueryFilter) -> Dynamic {
        let mut query = self.query_filtered(data, filter);
        let first = query.next();
        if first.is_unit() {
            return Dynamic::UNIT;
        }
        if !query.is_empty() {
            panic!("World::single_filtered expected at most one result, but query returned multiple results");
        }
        first
    }

    fn exists(&self, data: QueryData) -> bool {
        self.exists_filtered(data, QueryFilter::none())
    }

    fn exists_filtered(&self, data: QueryData, filter: QueryFilter) -> bool {
        let mut query = self.query_filtered(data, filter);
        !query.next().is_unit()
    }

    fn insert_resource(&self, resource_type_id: ImmutableString, payload: ImmutableString) {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_insert_dispatch(resource_type_id.as_str());
        dispatch(&mut world, payload.to_string());
        self.world.end_write(world);
    }

    fn init_resource(&self, resource_type_id: ImmutableString) {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_init_dispatch(resource_type_id.as_str());
        dispatch(&mut world);
        self.world.end_write(world);
    }

    fn remove_resource(&self, resource_type_id: ImmutableString) -> Dynamic {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_remove_dispatch(resource_type_id.as_str());
        let removed = dispatch(&mut world);
        self.world.end_write(world);

        removed.map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    }

    fn get_resource(&self, resource_type_id: ImmutableString) -> Dynamic {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_get_dispatch(resource_type_id.as_str());
        let value = dispatch(&mut world);
        self.world.end_write(world);

        value.map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    }

    fn get_resource_mut(&self, resource_type_id: ImmutableString) -> Dynamic {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_get_mut_dispatch(resource_type_id.as_str());
        let value = dispatch(&mut world);
        self.world.end_write(world);

        value.map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    }

    fn has_resource(&self, resource_type_id: ImmutableString) -> bool {
        let mut world = self.world.start_write();
        let dispatch = resolve_resource_get_dispatch(resource_type_id.as_str());
        let exists = dispatch(&mut world).is_some();
        self.world.end_write(world);
        exists
    }

    fn known_schedules(&self) -> Array {
        KNOWN_SCHEDULES.iter().map(|name| Dynamic::from((*name).to_string())).collect::<Array>()
    }

    fn run_schedule(&self, schedule_name: ImmutableString) -> bool {
        let mut world = self.world.start_write();
        let ran = run_known_schedule(&mut world, schedule_name.as_str());
        self.world.end_write(world);
        ran
    }

    fn write_message(&self, message_type_id: ImmutableString, payload: ImmutableString) {
        let mut world = self.world.start_write();
        let write_request = WriteMessageRequest::new(message_type_id.to_string(), payload.to_string());
        let write_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<crate::rhai_binding::value_semantics::access_cell::Scoped, ()> =
            unsafe { world.start_access(WORLD_ACCESS_METHOD_WRITE_MESSAGE, Box::new(write_request)) };
        unsafe { world.end_access(write_raw_handle) };

        self.world.end_write(world);
    }

    fn drain_messages(&self, message_type_id: ImmutableString) -> StringIter {
        let mut world = self.world.start_write();
        let drain_request = DrainMessagesRequest::new(message_type_id.to_string());
        let message_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            StringIter,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_DRAIN_MESSAGES, Box::new(drain_request)) };
        let messages = message_raw_handle.take();
        unsafe { world.end_access(message_raw_handle) };

        self.world.end_write(world);

        messages
    }

    fn write_probe_message(&self, payload: ImmutableString) {
        let mut world = self.world.start_write();
        let write_request = WriteProbeMessageRequest::new(payload.to_string());
        let write_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<crate::rhai_binding::value_semantics::access_cell::Scoped, ()> =
            unsafe { world.start_access(WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE, Box::new(write_request)) };
        unsafe { world.end_access(write_raw_handle) };

        self.world.end_write(world);
    }

    fn drain_probe_messages(&self) -> StringIter {
        let mut world = self.world.start_write();
        let message_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            StringIter,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES, Box::new(())) };
        let messages = message_raw_handle.take();
        unsafe { world.end_access(message_raw_handle) };

        self.world.end_write(world);

        messages
    }
}
