use spacetime_engine::{
    game::{LooCastDeveloperToolsPlugin, LooCastPlugin},
    run,
};

fn main() {
    run(|app| {
        app.add_plugins((LooCastPlugin, LooCastDeveloperToolsPlugin));
    });
}
