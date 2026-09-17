//! Tiny flat Developer Tools palette.
//!
//! F4 opens a short list of current visualizations. This is intentionally not a
//! generic settings renderer: domains register only `id + label + order + default`.

use bevy::prelude::*;

use crate::{
    input_focus::{InputFocus, InputFocusSet},
    ui::{UiTextRole, UiTheme},
    view::PrimaryViewPresentation,
};

use super::super::{DeveloperArtifact, DeveloperSet, DeveloperTools, VisualizationId};

const PALETTE_FOCUS_OWNER: &str = "devtools.tools_palette";

#[derive(Resource, Debug, Default)]
struct DeveloperToolsPaletteState {
    open: bool,
}

#[derive(Component)]
struct DeveloperToolsPaletteRoot;

#[derive(Component, Clone, Copy)]
struct VisualizationButton(VisualizationId);

#[derive(Component, Clone, Copy)]
struct VisualizationLabel(VisualizationId);

#[derive(Component)]
struct DeveloperMasterLabel;

pub(super) fn configure(app: &mut App) {
    app.init_resource::<DeveloperToolsPaletteState>()
        .init_resource::<InputFocus>()
        .add_systems(Startup, spawn_tools_palette)
        .add_systems(
            PreUpdate,
            handle_palette_keyboard.before(InputFocusSet::Resolve),
        )
        .add_systems(Update, handle_palette_buttons)
        .add_systems(
            PostUpdate,
            sync_tools_palette.in_set(DeveloperSet::RenderUi),
        );
}

fn spawn_tools_palette(mut commands: Commands, theme: Res<UiTheme>, tools: Res<DeveloperTools>) {
    let heading = theme.text(UiTextRole::Heading);
    let body = theme.text(UiTextRole::Compact);
    let secondary = theme.text(UiTextRole::Secondary);

    let mut specs = tools.visualizations().copied().collect::<Vec<_>>();
    specs.sort_by_key(|spec| (spec.order, spec.label));

    commands
        .spawn((
            Name::new("Developer Tools Palette"),
            DeveloperArtifact,
            DeveloperToolsPaletteRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                top: px(12.0),
                right: px(12.0),
                width: px(300.0),
                padding: UiRect::all(px(theme.panel_padding_px)),
                border: UiRect::all(px(1.0)),
                row_gap: px(theme.spacing_px),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.panel_background),
            BorderColor::all(theme.panel_border),
            GlobalZIndex(1_950),
        ))
        .with_children(|parent| {
            parent.spawn((
                DeveloperArtifact,
                Text::new("DEVELOPER TOOLS"),
                heading.font(),
                heading.color(),
            ));
            parent.spawn((
                DeveloperArtifact,
                DeveloperMasterLabel,
                Text::new(""),
                secondary.font(),
                secondary.color(),
            ));

            for spec in specs {
                parent
                    .spawn((
                        DeveloperArtifact,
                        Button,
                        VisualizationButton(spec.id),
                        Node {
                            width: percent(100.0),
                            padding: UiRect::axes(px(8.0), px(6.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.105, 0.115, 0.15, 0.94)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            DeveloperArtifact,
                            VisualizationLabel(spec.id),
                            Text::new(spec.label),
                            body.font(),
                            body.color(),
                        ));
                    });
            }
        });
}

fn handle_palette_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    presentation: Res<PrimaryViewPresentation>,
    mut state: ResMut<DeveloperToolsPaletteState>,
    mut focus: ResMut<InputFocus>,
) {
    if presentation.is_embedded() {
        state.open = false;
        focus.set_modal_claim(PALETTE_FOCUS_OWNER, false);
        return;
    }

    if keyboard.just_pressed(KeyCode::F4) {
        state.open = !state.open;
    } else if state.open && keyboard.just_pressed(KeyCode::Escape) {
        state.open = false;
    }

    focus.set_modal_claim(PALETTE_FOCUS_OWNER, state.open);
}

fn handle_palette_buttons(
    interactions: Query<(&Interaction, &VisualizationButton), (Changed<Interaction>, With<Button>)>,
    mut tools: ResMut<DeveloperTools>,
) {
    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed {
            tools.toggle_visualization(button.0);
        }
    }
}

fn sync_tools_palette(
    state: Res<DeveloperToolsPaletteState>,
    presentation: Res<PrimaryViewPresentation>,
    tools: Res<DeveloperTools>,
    mut roots: Query<&mut Node, With<DeveloperToolsPaletteRoot>>,
    mut buttons: Query<(&VisualizationButton, &mut BackgroundColor)>,
    mut texts: Query<(
        &mut Text,
        Option<&VisualizationLabel>,
        Option<&DeveloperMasterLabel>,
    )>,
) {
    for mut node in &mut roots {
        node.display = if state.open && !presentation.is_embedded() {
            Display::Flex
        } else {
            Display::None
        };
    }

    for (button, mut background) in &mut buttons {
        *background = BackgroundColor(if tools.visualization_selected(button.0) {
            Color::srgba(0.07, 0.28, 0.23, 0.95)
        } else {
            Color::srgba(0.105, 0.115, 0.15, 0.94)
        });
    }

    for (mut text, visualization, master) in &mut texts {
        if let Some(visualization) = visualization {
            if let Some(spec) = tools
                .visualizations()
                .find(|spec| spec.id == visualization.0)
            {
                text.0 = format!(
                    "[{}] {}",
                    if tools.visualization_selected(visualization.0) {
                        "x"
                    } else {
                        " "
                    },
                    spec.label,
                );
            }
        } else if master.is_some() {
            text.0 = format!(
                "F4 close  ·  F3 master {}",
                if tools.enabled() { "ON" } else { "OFF" },
            );
        }
    }
}
