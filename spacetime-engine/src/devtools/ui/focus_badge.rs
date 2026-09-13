//! Optional single screen-space badge for the current world focus.
//!
//! This is UI projected from a world point, not World Draw text. It is name-only
//! by design; detailed state belongs in the Inspector.

use bevy::prelude::*;

use crate::ui::{UiTextRole, UiTheme};

use super::super::{
    AppDeveloperToolsExt, DeveloperArtifact, DeveloperFocus, DeveloperSet, DeveloperTools,
    DeveloperView, VisualizationId, VisualizationSpec,
};

const FOCUS_BADGE: VisualizationId = VisualizationId("ui.focus_badge");

#[derive(Component)]
struct FocusBadgeRoot;

#[derive(Component)]
struct FocusBadgeText;

pub(super) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        FOCUS_BADGE,
        "Focus badge",
        0,
        false,
    ))
    .add_systems(Startup, spawn_focus_badge)
    .add_systems(
        PostUpdate,
        sync_focus_badge.in_set(DeveloperSet::RenderUi),
    );
}

fn spawn_focus_badge(mut commands: Commands, theme: Res<UiTheme>) {
    let tiny = theme.text(UiTextRole::Tiny);

    commands
        .spawn((
            Name::new("Developer Focus Badge"),
            DeveloperArtifact,
            FocusBadgeRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                border: UiRect::all(px(1.0)),
                padding: UiRect::axes(px(6.0), px(3.0)),
                ..default()
            },
            UiTransform::from_translation(Val2::percent(-50.0, -125.0)),
            BackgroundColor(theme.panel_background),
            BorderColor::all(theme.panel_border),
            GlobalZIndex(1_840),
        ))
        .with_children(|parent| {
            parent.spawn((
                DeveloperArtifact,
                FocusBadgeText,
                Text::new(""),
                tiny.font(),
                tiny.color(),
            ));
        });
}

fn sync_focus_badge(
    tools: Res<DeveloperTools>,
    focus: Res<DeveloperFocus>,
    view: Res<DeveloperView>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    names: Query<&Name>,
    mut roots: Query<&mut Node, With<FocusBadgeRoot>>,
    mut texts: Query<&mut Text, With<FocusBadgeText>>,
) {
    let Some(target) = tools
        .visualization_enabled(FOCUS_BADGE)
        .then(|| focus.current())
        .flatten()
    else {
        hide(&mut roots);
        return;
    };
    let Some(observer) = view.observer() else {
        hide(&mut roots);
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get(observer) else {
        hide(&mut roots);
        return;
    };
    let Ok(viewport_position) = camera.world_to_viewport(camera_transform, target.hit.position) else {
        hide(&mut roots);
        return;
    };

    for mut node in &mut roots {
        node.display = Display::Flex;
        node.left = px(viewport_position.x);
        node.top = px(viewport_position.y);
    }

    let name = names
        .get(target.semantic_entity)
        .or_else(|_| names.get(target.spatial_entity))
        .map(Name::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|_| format!("{:?}", target.semantic_entity));
    for mut text in &mut texts {
        text.0.clone_from(&name);
    }
}

fn hide(roots: &mut Query<&mut Node, With<FocusBadgeRoot>>) {
    for mut node in roots.iter_mut() {
        node.display = Display::None;
    }
}
