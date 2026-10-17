//! First-party Loo-Cast Game composition.

/// Stable identity of this Game content.
pub const GAME_ID: &str = "ghf-studios/loo-cast/loo-cast-game";

/// Human-readable title of this Game content.
pub fn title() -> &'static str {
    "Loo-Cast Game"
}

/// Install the complete Loo-Cast game and its developer adapters into an Engine app.
pub fn install(app: &mut spacetime_engine::EngineApp) {
    app.add_plugins((
        spacetime_engine::game::LooCastGamePlugin,
        spacetime_engine::game::LooCastDeveloperAdaptersPlugin,
    ));
}
