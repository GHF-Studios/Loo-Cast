// Modules
pub mod create_save_game_menu;
pub mod input_field;
pub mod main_menu;
pub mod pause_menu;
pub mod save_games_menu;

// Local imports
use create_save_game_menu::CreateSaveGameMenuPlugin;
use input_field::InputFieldPlugin;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use save_games_menu::SaveGamesMenuPlugin;

// External imports
use bevy::prelude::*;

// Constant variables
pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);
pub const PANEL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.5);

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const UNFOCUSED_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const FOCUSED_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

// Events
#[derive(Event)]
pub struct GainedFocus {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LostFocus {
    pub entity: Entity,
}

// Resources
#[derive(Resource)]
pub struct UIManager {
    pub focus: Option<Entity>,
}

// Structs
pub struct UIPlugin;

// Implementations
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GainedFocus>()
            .add_event::<LostFocus>()
            // Plugins
            .add_plugins((
                CreateSaveGameMenuPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                SaveGamesMenuPlugin,
                InputFieldPlugin,
            ))
            // Startup Systems
            .add_systems(Startup, UIManager::initialize)
            // Update Systems
            .add_systems(Update, UIManager::handle_gained_focus_event);
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self { focus: None }
    }
}

impl UIManager {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(UIManager {focus: None});
    }

    fn handle_gained_focus_event(
        mut ui_manager: ResMut<UIManager>,
        mut gained_focus_event_reader: EventReader<GainedFocus>,
        mut lost_focus_event_writer: EventWriter<LostFocus>,
    ) {
        if let Some(gained_focus_event) = gained_focus_event_reader.iter().last() {
            if let Some(old_focus) = ui_manager.focus {
                lost_focus_event_writer.send(LostFocus { entity: old_focus });
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