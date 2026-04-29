//// Rightful author and owner of this holy manuscript (seriously dude you saved my ass, i hope this "message in a bottle" (rather message in a commit lol) reaches you in the far far future sometime you absolutely don't expect this and it'll be fucking funny and you will hate me for the fact that I've written all of this into a singular giant fucking line)


use bevy::prelude::*;
use bevy::scene::serde::{SceneDeserializer, SceneSerializer};

fn main() {
    println!("Hello, world!");

    let mut app = App::new();

    // Need to register types in the type registry, so that they can appear in the output DynamicScene
    app.add_plugins(TypeRegistrationPlugin);
    app.add_plugins(TransformPlugin);

    app.world
        .spawn(Transform::from_xyz(0.0, 0.0, 0.0));
    app.world
        .spawn(Transform::from_xyz(1.0, 1.0, 1.0));
    app.world
        .spawn(Transform::from_xyz(2.0, 2.0, 2.0));

    let entities = app
        .world
        .query_filtered::<Entity, With<Transform>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
    let mut builder = DynamicSceneBuilder::from_world(&app.world);

    builder = builder.extract_entities(entities.into_iter());

    let dyn_scene = builder.build();

    // Now to serialize it...

    let type_registry_arc = &app.world.resource::<AppTypeRegistry>().0;

    let serializer = SceneSerializer::new(&dyn_scene, type_registry_arc);

    // SceneSerializer implements Serde's traits, so we can just yoink it lol
    let serialized = ron::to_string(&serializer).unwrap();

    println!("Serialized!\n{}\n", serialized);

    // Ok, now let's deserialize it.
    // Start from scratch...

    drop(app);
    let mut app = App::new();

    // Need to register types in the type registry, so that they can be deserialized from the DynamicScene
    app.add_plugins(TypeRegistrationPlugin);
    app.add_plugins(TransformPlugin);

    let type_registry_rwlock = app.world.resource::<AppTypeRegistry>().0.read();

    let deserializer = SceneDeserializer {
        type_registry: &type_registry_rwlock,
    };

    let mut ron_deserializer = ron::de::Deserializer::from_str(&serialized).unwrap();

    use serde::de::DeserializeSeed;

    let dyn_scene = deserializer.deserialize(&mut ron_deserializer).unwrap();

    println!("Deserialized! {} entities.", dyn_scene.entities.len());

    drop(type_registry_rwlock);

    dyn_scene
        .write_to_world(&mut app.world, &mut default())
        .unwrap();

    println!("Written to world!");

    // We wrote it to the new world.
    for (entity, transform) in app
        .world
        .query::<(Entity, &Transform)>()
        .iter(&app.world)
    {
        println!("Entity {:?}:\n{:?}", entity, transform);
    }
}
