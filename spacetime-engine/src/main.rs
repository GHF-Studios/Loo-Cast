//! Desktop entrypoint: selects and runs the Loo Cast game through the engine host.

use spacetime_engine::{
    game::{LooCastDeveloperAdaptersPlugin, LooCastGamePlugin},
    run,
};

fn main() {
    run(|app| {
        app.add_plugins((LooCastGamePlugin, LooCastDeveloperAdaptersPlugin));
    });
}
