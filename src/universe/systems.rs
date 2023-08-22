use super::resources::*;

use bevy::prelude::*;
use serde::*;
use serde_json::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    let serialized_universe = serde_json::to_string(&*universe_resource).unwrap();

    let path = Path::new("data/saves/universe.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(serialized_universe.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn delete_universe() {

}

pub fn load_universe(mut commands: Commands) {
    let path = Path::new("data/saves/universe.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut serialized_universe = String::new();
    match file.read_to_string(&mut serialized_universe) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("successfully read {}", display),
    }

    let universe: Universe = serde_json::from_str(&serialized_universe).unwrap();

    commands.insert_resource(universe);
}

pub fn unload_universe() {

}

pub fn print_universe(universe_resource: Res<Universe>) {
    println!("Universe: {}", universe_resource.name);
}