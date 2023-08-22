use super::resources::*;

use bevy::prelude::*;
use serde::*;
use serde_json::*;

pub fn create_universe(
    mut commands: Commands,
) {
    commands.insert_resource(Universe {
        name: "Default Universe".to_string()
    });
}

pub fn save_universe(
    universe_resource: Res<Universe>
) {
    println!("Creating new universe '{}'", universe_resource.name);
    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&*universe_resource).unwrap();
    
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
}

pub fn delete_universe() {

}

pub fn load_universe() {

}

pub fn unload_universe() {

}