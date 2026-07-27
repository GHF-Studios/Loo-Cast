use std::{env, path::PathBuf};

fn main() {
    println!("Vapor engine content placeholder");
    println!();
    println!("Status");

    if let Some(engine) = env::var_os("VAPOR_ENGINE_ID") {
        println!("  engine id: {}", engine.to_string_lossy());
    }
    if let Some(packagepack) = env::var_os("VAPOR_PACKAGEPACK_ID") {
        println!("  packagepack id: {}", packagepack.to_string_lossy());
    }
    if let Some(game) = env::var_os("VAPOR_GAME_ID") {
        println!("  game id: {}", game.to_string_lossy());
    }
    if let Some(request) = env::var_os("VAPOR_LAUNCH_REQUEST") {
        println!("  launch request: {}", request.to_string_lossy());
    }
    if let Some(root) = env::var_os("VAPOR_ENGINE_ROOT").map(PathBuf::from) {
        println!("  engine root: {}", root.display());
    }
    if let Some(target) = env::var_os("VAPOR_RUNTIME_TARGET") {
        println!("  runtime target: {}", target.to_string_lossy());
    }

    println!();
    println!("Handoff");
    println!("  Runtime implementation is not available in this placeholder yet.");
    println!("  This executable only proves the packagepack-to-engine handoff contract.");
}
