use std::{env, path::PathBuf};

fn main() {
    println!("Spacetime Engine");
    println!();
    println!("Status");
    println!("  First-party Loo-Cast engine content is installed.");

    if let Some(packagepack) = env::var_os("VAPOR_PACKAGEPACK_ID") {
        println!("  Packagepack: {}", packagepack.to_string_lossy());
    }
    if let Some(game) = env::var_os("VAPOR_LAUNCH_TARGET") {
        println!("  Launch target: {}", game.to_string_lossy());
    }
    if let Some(root) = env::var_os("VAPOR_ENGINE_ROOT").map(PathBuf::from) {
        println!("  Engine root: {}", root.display());
    }
    if let Some(target) = env::var_os("VAPOR_RUNTIME_TARGET") {
        println!("  Runtime target: {}", target.to_string_lossy());
    }

    println!();
    println!("Handoff");
    println!("  Loo-Cast product runtime is not implemented in this placeholder yet.");
    println!("  The terminal runtime proof now lives in Vapor-Examples.");
}
