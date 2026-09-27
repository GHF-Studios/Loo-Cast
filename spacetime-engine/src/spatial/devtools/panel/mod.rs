//! Runtime UI panel for bounded local/canonical spatial state.

use super::*;

#[derive(Component)]
pub(super) struct UsfSpatialDebugRoot;

#[derive(Component)]
pub(super) struct UsfSpatialDebugText;

pub(super) fn spawn_debug_panel(mut commands: Commands, theme: Res<UiTheme>) {
    let heading = theme.text(UiTextRole::Heading);
    let compact = theme.text(UiTextRole::Compact);

    commands
        .spawn((
            Name::new("USF Spatial Debug"),
            DeveloperArtifact,
            UsfSpatialDebugRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                left: px(12.0),
                bottom: px(12.0),
                width: px(620.0),
                padding: UiRect::all(px(theme.panel_padding_px)),
                border: UiRect::all(px(1.0)),
                row_gap: px(theme.spacing_px),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.panel_background),
            BorderColor::all(theme.panel_border),
            GlobalZIndex(1_940),
        ))
        .with_children(|parent| {
            parent.spawn((
                DeveloperArtifact,
                Text::new("USF SPATIAL"),
                heading.font(),
                heading.color(),
            ));
            parent.spawn((
                DeveloperArtifact,
                UsfSpatialDebugText,
                Text::new(""),
                compact.font(),
                compact.color(),
            ));
        });
}

pub(super) fn update_debug_panel(
    tools: Res<DeveloperTools>,
    frame: Res<UsfSpatialFrame>,
    ownership: UsfOwnershipQuery,
    anchors: Query<
        (Entity, &Transform, Option<&LinearVelocity>, &UsfScaleLayer),
        With<UsfSpatialAnchor>,
    >,
    semantic_positions: Query<&UsfPosition>,
    mut roots: Query<&mut Node, With<UsfSpatialDebugRoot>>,
    mut texts: Query<&mut Text, With<UsfSpatialDebugText>>,
) {
    let enabled = tools.visualization_enabled(USF_SPATIAL_VISUALIZATION);
    for mut node in &mut roots {
        node.display = if enabled {
            Display::Flex
        } else {
            Display::None
        };
    }
    if !enabled {
        return;
    }

    let Some((realization, transform, velocity, layer)) = anchors.iter().next() else {
        for mut text in &mut texts {
            text.0 = "No USF spatial anchor is active.".to_string();
        }
        return;
    };

    let semantic = ownership
        .semantic_of(realization)
        .and_then(|semantic| semantic_positions.get(semantic).ok());
    let velocity = velocity.map_or(Vec3::ZERO, |velocity| velocity.0);
    let semantic_text = semantic
        .map(UsfPosition::format_stack)
        .unwrap_or_else(|| "<missing semantic UsfPosition>".to_string());

    let scale = layer.scale();
    let metres_per_native = scale.metres_per_native();
    let velocity_metres = velocity * metres_per_native as f32;

    let output = format!(
        "Runtime scale: S{} (1 native = {:.3e} m)\n\
Local position: ({:.6}, {:.6}, {:.6}) native\n\
Local velocity: ({:.6}, {:.6}, {:.6}) native/s  |v|={:.6}\n\
Physical velocity: ({:.3}, {:.3}, {:.3}) m/s  |v|={:.3}\n\
Semantic position: {}\n\
Frame origin: {}\n\
Rebases: {}  last shift=({:.6}, {:.6}, {:.6}) native@S{}",
        scale,
        metres_per_native,
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
        velocity.x,
        velocity.y,
        velocity.z,
        velocity.length(),
        velocity_metres.x,
        velocity_metres.y,
        velocity_metres.z,
        velocity_metres.length(),
        semantic_text,
        frame.origin().format_stack(),
        frame.rebase_count(),
        frame.last_shift().x,
        frame.last_shift().y,
        frame.last_shift().z,
        scale,
    );

    for mut text in &mut texts {
        text.0.clone_from(&output);
    }
}
