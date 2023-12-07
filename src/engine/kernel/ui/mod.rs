// Modules
pub mod input_field;
pub mod main_menu;
pub mod pause_menu;
pub mod save_game_creation_menu;
pub mod save_games_menu;

// Local imports
use input_field::InputFieldPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use save_game_creation_menu::SaveGameCreationMenuPlugin;
use save_games_menu::SaveGamesMenuPlugin;

// Internal imports

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables
pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);
pub const PANEL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.5);

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const UNFOCUSED_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const FOCUSED_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

// Types

// Enums

// Structs
pub struct UIPlugin;

#[derive(Event)]
pub struct GainFocus {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LoseFocus {
    pub entity: Entity,
}

#[derive(Resource)]
#[derive(Default)]
pub struct UIManager {
    pub focus: Option<Entity>,
}

// Implementations
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GainFocus>()
            .add_event::<LoseFocus>()
            // Plugins
            .add_plugins((
                SaveGameCreationMenuPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                SaveGamesMenuPlugin,
                InputFieldPlugin,
            ))
            // Startup Systems
            .add_systems(Startup, UIManager::initialize)
            // Update Systems
            .add_systems(Update, UIManager::handle_gain_focus);
    }
}



impl UIManager {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(UIManager { focus: None });
    }

    fn handle_gain_focus(
        mut ui_manager: ResMut<UIManager>,
        mut gained_focus_event_reader: EventReader<GainFocus>,
        mut lost_focus_event_writer: EventWriter<LoseFocus>,
    ) {
        if let Some(gained_focus_event) = gained_focus_event_reader.iter().last() {
            if let Some(old_focus) = ui_manager.focus {
                lost_focus_event_writer.send(LoseFocus { entity: old_focus });
            }
            ui_manager.focus = Some(gained_focus_event.entity);
        }
    }

    pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 80.0,
            color: Color::WHITE,
        }
    }

    pub fn get_label_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 48.0,
            color: Color::WHITE,
        }
    }

    pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            color: Color::WHITE,
        }
    }
}

// Module Functions
