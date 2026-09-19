use spacetime_engine::{
    game::{TestGameDeveloperToolsPlugin, TestGamePlugin},
    run,
};

fn main() {
    run(|app| {
        app.add_plugins((TestGamePlugin, TestGameDeveloperToolsPlugin));
    });
}
