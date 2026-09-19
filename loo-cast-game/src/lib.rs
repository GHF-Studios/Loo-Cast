//! First-party Loo Cast Game.
//!
//! The implementation is still temporarily housed under
//! `spacetime_engine::game` while the runtime/composition boundary is proven.
//! Current Vapor already composes this crate as the effective Game, so the
//! physical module relocation can follow without changing the host contract.

pub const GAME_ID: &str = "ghf-studios/loo-cast/loo-cast-game";

pub fn title() -> &'static str {
    "Loo-Cast Game"
}

/// Install Loo Cast into its Spacetime Engine application.
///
/// This function is the current Spacetime-Engine-specific Game integration
/// surface consumed by Vapor's generated static composition.
pub fn install(app: &mut spacetime_engine::EngineApp) {
    app.add_plugins((
        spacetime_engine::game::TestGamePlugin,
        spacetime_engine::game::TestGameDeveloperToolsPlugin,
    ));
}
