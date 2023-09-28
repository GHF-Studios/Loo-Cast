// Internal imports
use crate::ui::*;

// External imports
use bevy::{
    prelude::*,
    text::TextLayoutInfo,
    ui::{widget::TextFlags, ContentSize, FocusPolicy},
};

// Bundles
#[derive(Bundle, Debug)]
pub struct InputFieldBundle {
    pub node: Node,
    pub input_field: InputField,
    pub button: Button,
    pub style: Style,
    pub interaction: Interaction,
    pub text: Text,
    pub text_layout_info: TextLayoutInfo,
    pub text_flags: TextFlags,
    pub calculated_size: ContentSize,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub image: UiImage,
}

// Components
#[derive(Component, Debug)]
pub struct InputField {
    pub value: String,
}

// Resources
#[derive(Resource)]
pub struct InputFieldManager;

// Structs
pub struct InputFieldPlugin;

// Implementations
impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(Startup, InputFieldManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    InputFieldManager::text_input_system,
                    InputFieldManager::text_render_system,
                    InputFieldManager::interact_with_input_field,
                    InputFieldManager::handle_gained_focus_event,
                    InputFieldManager::handle_lost_focus_event,
                ),
            );
    }
}

impl Default for InputFieldBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            input_field: InputField::default(),
            button: Button::default(),
            style: Style::default(),
            interaction: Interaction::default(),
            text: Text::default(),
            text_layout_info: TextLayoutInfo::default(),
            text_flags: TextFlags::default(),
            calculated_size: ContentSize::default(),
            focus_policy: FocusPolicy::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            z_index: ZIndex::default(),
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            border_color: BorderColor::default(),
            image: UiImage::default(),
        }
    }
}

impl Default for InputField {
    fn default() -> Self {
        Self {
            value: String::new(),
        }
    }
}

impl InputFieldManager {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(InputFieldManager {});
    }

    fn text_input_system(
        mut received_character_event_reader: EventReader<ReceivedCharacter>,
        keyboard_input: Res<Input<KeyCode>>,
        ui_manager: Res<UIManager>,
        mut input_field_query: Query<(&mut InputField, Entity)>,
    ) {
        for (mut input_field, input_field_entity) in input_field_query.iter_mut() {
            if Some(input_field_entity) != ui_manager.focus {
                continue;
            }

            if keyboard_input.just_pressed(KeyCode::Back) {
                if !input_field.value.is_empty() {
                    input_field.value.pop();
                }
                continue;
            }

            for received_char_event in received_character_event_reader.iter() {
                let received_char = received_char_event.char;
                if received_char.is_alphanumeric() || received_char.is_whitespace() {
                    input_field.value.push(received_char);
                }
            }
        }
    }

    fn text_render_system(mut input_field_query: Query<(&InputField, &mut Text)>) {
        for (input_field, mut input_field_text) in input_field_query.iter_mut() {
            input_field_text.sections[0].value = input_field.value.clone();
        }
    }

    fn interact_with_input_field(
        mut gained_focus_event_writer: EventWriter<GainedFocus>,
        ui_manager: Res<UIManager>,
        mut input_field_query: Query<
            (&Interaction, Entity),
            (Changed<Interaction>, With<InputField>),
        >,
    ) {
        if let Ok((interaction, input_field_entity)) = input_field_query.get_single_mut() {
            match *interaction {
                Interaction::Pressed => {
                    if ui_manager.focus != Some(input_field_entity) {
                        gained_focus_event_writer.send(GainedFocus {
                            entity: input_field_entity,
                        });
                    }
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }

    fn handle_gained_focus_event(
        mut gained_focus_event_reader: EventReader<GainedFocus>,
        mut input_field_query: Query<(&mut BackgroundColor, Entity), With<InputField>>,
    ) {
        if let Some(gained_focus_event) = gained_focus_event_reader.iter().last() {
            if let Ok((mut background_color, input_field_entity)) =
                input_field_query.get_mut(gained_focus_event.entity)
            {
                if gained_focus_event.entity == input_field_entity {
                    *background_color = FOCUSED_COLOR.into();
                }
            }
        }
    }

    fn handle_lost_focus_event(
        mut lost_focus_event_reader: EventReader<LostFocus>,
        mut input_field_query: Query<(&mut BackgroundColor, Entity), With<InputField>>,
    ) {
        if let Some(lost_focus_event) = lost_focus_event_reader.iter().last() {
            if let Ok((mut background_color, input_field_entity)) =
                input_field_query.get_mut(lost_focus_event.entity)
            {
                if lost_focus_event.entity == input_field_entity {
                    *background_color = UNFOCUSED_COLOR.into();
                }
            }
        }
    }
}
