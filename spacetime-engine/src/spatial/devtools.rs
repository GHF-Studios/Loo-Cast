use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    devtools::{
        AppDeveloperToolsExt, DeveloperArtifact, DeveloperSet, DeveloperTools, VisualizationId,
        VisualizationSpec,
    },
    ecs::UsfManifestationOf,
    ui::{UiTextRole, UiTheme},
};

use super::{UsfPosition, UsfSpatialAnchor, UsfSpatialFrame, UsfSpatialSet};

const USF_SPATIAL_VISUALIZATION: VisualizationId = VisualizationId("usf_spatial");

#[derive(Component)]
struct UsfSpatialDebugRoot;

#[derive(Component)]
struct UsfSpatialDebugText;

pub(super) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        USF_SPATIAL_VISUALIZATION,
        "USF Spatial",
        25,
        false,
    ))
    .add_systems(Startup, spawn_debug_panel)
    .add_systems(
        PostUpdate,
        update_debug_panel
            .after(UsfSpatialSet::Rebase)
            .in_set(DeveloperSet::RenderUi),
    );
}

fn spawn_debug_panel(mut commands: Commands, theme: Res<UiTheme>) {
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

fn update_debug_panel(
    tools: Res<DeveloperTools>,
    frame: Res<UsfSpatialFrame>,
    anchors: Query<(&Transform, Option<&LinearVelocity>, &UsfManifestationOf), With<UsfSpatialAnchor>>,
    semantic_positions: Query<&UsfPosition>,
    mut roots: Query<&mut Node, With<UsfSpatialDebugRoot>>,
    mut texts: Query<&mut Text, With<UsfSpatialDebugText>>,
) {
    let enabled = tools.visualization_enabled(USF_SPATIAL_VISUALIZATION);
    for mut node in &mut roots {
        node.display = if enabled { Display::Flex } else { Display::None };
    }
    if !enabled {
        return;
    }

    let Some((transform, velocity, manifestation)) = anchors.iter().next() else {
        for mut text in &mut texts {
            text.0 = "No USF spatial anchor is active.".to_string();
        }
        return;
    };

    let semantic = semantic_positions.get(manifestation.0).ok();
    let velocity = velocity.map_or(Vec3::ZERO, |velocity| velocity.0);
    let semantic_text = semantic
        .map(UsfPosition::format_stack)
        .unwrap_or_else(|| "<missing semantic UsfPosition>".to_string());

    let output = format!(
        "Runtime scale: S0 (1 unit = 1 metre)\n\
Local position: ({:.3}, {:.3}, {:.3}) m\n\
Local velocity: ({:.3}, {:.3}, {:.3}) m/s  |v|={:.3}\n\
Semantic position: {}\n\
Frame origin: {}\n\
Rebases: {}  last shift=({:.1}, {:.1}, {:.1}) m",
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
        velocity.x,
        velocity.y,
        velocity.z,
        velocity.length(),
        semantic_text,
        frame.origin().format_stack(),
        frame.rebase_count(),
        frame.last_shift().x,
        frame.last_shift().y,
        frame.last_shift().z,
    );

    for mut text in &mut texts {
        text.0 = output.clone();
    }
}
