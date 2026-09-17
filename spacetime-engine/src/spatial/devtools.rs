use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    devtools::{
        AppDeveloperToolsExt, DeveloperArtifact, DeveloperSet, DeveloperTools, DrawDepth,
        VisualizationId, VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
    },
    ecs::UsfManifestationOf,
    ui::{UiTextRole, UiTheme},
};

use super::{SpatialDemandSnapshot, UsfPosition, UsfSpatialAnchor, UsfSpatialFrame, UsfSpatialSet};

const USF_SPATIAL_VISUALIZATION: VisualizationId = VisualizationId("usf_spatial");
pub(crate) const SPATIAL_DEMAND_VISUALIZATION: VisualizationId =
    VisualizationId("usf_spatial_demand");
const DEMAND_DEBUG_PROJECTION_BOUND: f32 = 1_000_000.0;

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
    .register_developer_visualization(VisualizationSpec::new(
        SPATIAL_DEMAND_VISUALIZATION,
        "Spatial demand / materialization",
        26,
        false,
    ))
    .add_systems(Startup, spawn_debug_panel)
    .add_systems(
        PostUpdate,
        update_debug_panel
            .after(UsfSpatialSet::Rebase)
            .in_set(DeveloperSet::RenderUi),
    )
    .add_systems(
        PostUpdate,
        collect_spatial_demand_world_draw.in_set(DeveloperSet::CollectWorldDraw),
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
    anchors: Query<
        (&Transform, Option<&LinearVelocity>, &UsfManifestationOf),
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

fn collect_spatial_demand_world_draw(
    tools: Res<DeveloperTools>,
    spatial_frame: Res<UsfSpatialFrame>,
    demands: Res<SpatialDemandSnapshot>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(SPATIAL_DEMAND_VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();
    let color = Color::srgba(0.20, 0.82, 1.0, 0.92);

    for demand in demands.iter() {
        let Ok(center) = demand
            .center()
            .relative_native_bounded(spatial_frame.origin(), DEMAND_DEBUG_PROJECTION_BOUND)
        else {
            continue;
        };
        let half = demand.half_extent_native();
        draw_wire_box(&mut batch, center - half, center + half, color);
        batch.cross(
            Isometry3d::new(center, Quat::IDENTITY),
            0.45,
            color,
            DrawDepth::Overlay,
        );
    }

    frame.submit(batch);
}

fn draw_wire_box(batch: &mut WorldDrawBatch, min: Vec3, max: Vec3, color: Color) {
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, max.y, max.z),
    ];
    for (a, b) in [
        (0, 1),
        (0, 2),
        (1, 3),
        (2, 3),
        (4, 5),
        (4, 6),
        (5, 7),
        (6, 7),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ] {
        batch.line(corners[a], corners[b], color, DrawDepth::Overlay);
    }
}
