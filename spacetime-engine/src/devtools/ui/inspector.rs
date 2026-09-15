//! Compact Bevy-UI semantic Inspector for the canonical tooling focus.
//!
//! The embedded editor uses the richer egui widgets from `inspect_ui`; this
//! retained surface intentionally stays text-only for immersive developer use.

use bevy::prelude::*;

use crate::{
    ui::{UiTextRole, UiTheme},
    view::PrimaryViewPresentation,
};

use super::super::{
    inspect_ui, DeveloperArtifact, DeveloperFocus, DeveloperSet, DeveloperTools, InspectAccess,
    InspectNumberFormat, InspectionFrame, StructureItemId, StructureSelection,
};

#[derive(Component)]
struct DeveloperInspectorRoot;

#[derive(Component)]
struct DeveloperInspectorTitle;

#[derive(Component)]
struct DeveloperInspectorBody;

pub(super) fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_inspector).add_systems(
        PostUpdate,
        sync_inspector.in_set(DeveloperSet::RenderUi),
    );
}

fn spawn_inspector(mut commands: Commands, theme: Res<UiTheme>) {
    let heading = theme.text(UiTextRole::Heading);
    let body = theme.text(UiTextRole::Compact);

    commands
        .spawn((
            Name::new("Developer Inspector"),
            DeveloperArtifact,
            DeveloperInspectorRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                top: px(12.0),
                left: px(12.0),
                width: px(360.0),
                max_height: percent(88.0),
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(px(theme.panel_padding_px)),
                border: UiRect::all(px(1.0)),
                row_gap: px(theme.spacing_px),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.panel_background),
            BorderColor::all(theme.panel_border),
            GlobalZIndex(1_850),
        ))
        .with_children(|parent| {
            parent.spawn((
                DeveloperArtifact,
                DeveloperInspectorTitle,
                Text::new("INSPECTOR"),
                heading.font(),
                heading.color(),
            ));
            parent.spawn((
                DeveloperArtifact,
                DeveloperInspectorBody,
                Text::new(""),
                body.font(),
                body.color(),
            ));
        });
}

fn sync_inspector(
    tools: Res<DeveloperTools>,
    presentation: Res<PrimaryViewPresentation>,
    focus: Res<DeveloperFocus>,
    structure: Res<StructureSelection>,
    frame: Res<InspectionFrame>,
    names: Query<&Name>,
    mut roots: Query<&mut Node, With<DeveloperInspectorRoot>>,
    mut texts: Query<(
        &mut Text,
        Option<&DeveloperInspectorTitle>,
        Option<&DeveloperInspectorBody>,
    )>,
) {
    let target = (!presentation.is_embedded() && tools.enabled())
        .then(|| focus.current())
        .flatten();
    let visible = target.is_some();

    for mut node in &mut roots {
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }

    let Some(target) = target else {
        return;
    };

    let semantic_name = names
        .get(target.semantic_entity)
        .map(Name::as_str)
        .or_else(|_| names.get(target.spatial_entity).map(Name::as_str))
        .unwrap_or("Unnamed entity");

    let title = if focus.selected().is_some() {
        format!("{semantic_name}  ·  SELECTED")
    } else {
        semantic_name.to_owned()
    };
    let body = render_body(
        focus.selected().is_some(),
        target.hit,
        structure.item_for(target),
        true,
        &frame,
    );

    for (mut text, title_marker, body_marker) in &mut texts {
        if title_marker.is_some() {
            text.0.clone_from(&title);
        } else if body_marker.is_some() {
            text.0.clone_from(&body);
        }
    }
}

pub(super) fn render_body(
    selected: bool,
    hit: Option<super::super::FocusHit>,
    scope: Option<StructureItemId>,
    show_pin_hint: bool,
    frame: &InspectionFrame,
) -> String {
    let mut lines = Vec::<String>::new();
    let source = if selected { "selected" } else { "hover focus" };
    if let Some(hit) = hit {
        let distance = inspect_ui::format_quantity(
            hit.distance_meters as f64,
            "m",
            InspectNumberFormat::significant_digits(4),
        );
        if show_pin_hint {
            lines.push(format!(
                "{source}  ·  {distance}  ·  P {}",
                if selected { "clear selection" } else { "select" },
            ));
        } else {
            lines.push(format!("{source}  ·  {distance}"));
        }
    } else if show_pin_hint && selected {
        lines.push(format!("{source}  ·  P clear selection"));
    } else {
        lines.push(source.to_owned());
    }

    let sections = frame.sorted_sections_for(scope);
    if sections.is_empty() {
        lines.push(String::new());
        lines.push("No semantic inspection data for this Structure item.".to_owned());
    }

    for section in sections {
        lines.push(String::new());
        lines.push(section.title.to_uppercase());
        for field in &section.fields {
            let label = field
                .symbol
                .map(|symbol| format!("{} ({symbol})", field.label))
                .unwrap_or_else(|| field.label.clone());
            let access = if field.access == InspectAccess::ReadOnly {
                String::new()
            } else {
                format!("  [{}]", field.access.label())
            };
            lines.push(format!(
                "{label}: {}{access}",
                inspect_ui::format_value(&field.value)
            ));
        }
    }

    lines.join("\n")
}
