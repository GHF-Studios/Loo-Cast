use crate::ui::input_field::components::*;

use bevy::{
    prelude::*,
    text::TextLayoutInfo,
    ui::{widget::TextFlags, ContentSize, FocusPolicy},
};

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
