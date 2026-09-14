//! Compact semantic Inspector for the canonical [`DeveloperFocus`].
//!
//! The same structured inspection frame feeds immersive developer UI and the
//! embedded editor. Structure selection may refine the editor to one semantic
//! section without changing the concrete ECS entity inspected by the raw ECS
//! Inspector.

use bevy::prelude::*;

use crate::{
    ui::{UiTextRole, UiTheme},
    view::PrimaryViewPresentation,
};

use super::super::{
    DeveloperArtifact, DeveloperFocus, DeveloperSet, DeveloperTools, InspectAccess,
    InspectNumberFormat, InspectSectionId, InspectValue, InspectionFrame,
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
    frame: Res<InspectionFrame>,
    names: Query<&Name>,
    mut roots: Query<&mut Node, With<DeveloperInspectorRoot>>,
    mut texts: Query<(
        &mut Text,
        Option<&DeveloperInspectorTitle>,
        Option<&DeveloperInspectorBody>,
    )>,
) {
    // The compact Bevy-UI inspector remains useful as an immersive debug
    // surface. Embedded presentation uses the docked semantic Inspector instead.
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

    let title = if focus.pinned().is_some() {
        format!("{semantic_name}  ·  PINNED")
    } else {
        semantic_name.to_owned()
    };
    let body = render_body(
        focus.pinned().is_some(),
        target.hit.map(|hit| hit.distance_meters),
        &frame,
        None,
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
    pinned: bool,
    distance_meters: Option<f32>,
    frame: &InspectionFrame,
    selected_section: Option<InspectSectionId>,
) -> String {
    let mut lines = Vec::<String>::new();
    let focus_kind = if pinned { "pinned" } else { "focus" };
    let distance = distance_meters
        .map(|distance| {
            format_quantity(
                distance as f64,
                "m",
                InspectNumberFormat::significant_digits(4),
            )
        })
        .unwrap_or_else(|| "non-spatial selection".to_owned());
    lines.push(format!(
        "{focus_kind}  ·  {distance}  ·  P {}",
        if pinned { "unpin" } else { "pin" },
    ));

    for section in frame.sorted_sections_matching(selected_section) {
        lines.push(String::new());
        lines.push(section.title.to_uppercase());
        for field in &section.fields {
            let label = field
                .symbol
                .map(|symbol| format!("{} ({symbol})", field.label))
                .unwrap_or_else(|| field.label.clone());
            lines.push(format!(
                "{label}: {}{}",
                format_value(&field.value),
                access_suffix(field.access),
            ));
        }
    }

    if selected_section.is_some() && lines.len() == 1 {
        lines.push(String::new());
        lines.push("Selected Structure item has no inspection output this frame.".to_owned());
    }

    lines.join("\n")
}

pub(in crate::devtools) fn draw_editor_inspector(
    ui: &mut bevy_egui::egui::Ui,
    tools: &DeveloperTools,
    target: Option<super::super::FocusTarget>,
    focus_name: &str,
    pinned: bool,
    frame: &InspectionFrame,
    selected_section: Option<InspectSectionId>,
) {
    ui.heading(focus_name);

    if !tools.enabled() {
        ui.weak("Developer output is disabled. Enable it here or press F3.");
        return;
    }

    let Some(target) = target else {
        ui.weak("No inspectable focus.");
        ui.label("Select an entity in Hierarchy or click one in the Game view.");
        return;
    };

    ui.monospace(render_body(
        pinned,
        target.hit.map(|hit| hit.distance_meters),
        frame,
        selected_section,
    ));
}

fn access_suffix(access: InspectAccess) -> &'static str {
    match access {
        InspectAccess::ReadOnly => "",
        InspectAccess::Direct => "  [direct]",
        InspectAccess::Validated => "  [validated]",
        InspectAccess::Transactional => "  [transactional]",
        InspectAccess::Command => "  [command]",
    }
}

fn format_value(value: &InspectValue) -> String {
    match value {
        InspectValue::Text(value) => value.clone(),
        InspectValue::Bool(value) => (if *value { "yes" } else { "no" }).to_owned(),
        InspectValue::Integer(value) => value.to_string(),
        InspectValue::Number { value, format } => format_number(*value, *format),
        InspectValue::Quantity {
            value,
            unit,
            format,
        } => format_quantity(*value, unit.0, *format),
        InspectValue::Range {
            minimum,
            maximum,
            unit,
            format,
        } => format!(
            "{} … {}",
            format_quantity(*minimum, unit.0, *format),
            format_quantity(*maximum, unit.0, *format),
        ),
        InspectValue::Entity(entity) => format!("{entity:?}"),
        InspectValue::Vec3(value) => format!("({:.3}, {:.3}, {:.3})", value.x, value.y, value.z),
    }
}

fn format_quantity(value: f64, unit: &str, format: InspectNumberFormat) -> String {
    format!("{} {unit}", format_number(value, format))
}

fn format_number(value: f64, format: InspectNumberFormat) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    if value == 0.0 {
        return "0".to_owned();
    }

    let significant_digits = usize::from(format.significant_digits.clamp(1, 12));
    let magnitude = value.abs();
    let exponent = magnitude.log10().floor() as i32;

    if exponent >= 6 || exponent <= -4 {
        let engineering_exponent = exponent.div_euclid(3) * 3;
        let scaled = value / 10.0_f64.powi(engineering_exponent);
        let scaled_exponent = scaled.abs().log10().floor() as i32;
        let decimals = (significant_digits as i32 - 1 - scaled_exponent).clamp(0, 10) as usize;
        return format!(
            "{} × 10{}",
            trim_decimal_zeros(format!("{scaled:.decimals$}")),
            superscript_integer(engineering_exponent),
        );
    }

    let decimals = (significant_digits as i32 - 1 - exponent).clamp(0, 10) as usize;
    trim_decimal_zeros(format!("{value:.decimals$}"))
}

fn trim_decimal_zeros(mut value: String) -> String {
    if !value.contains('.') {
        return value;
    }
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    value
}

fn superscript_integer(value: i32) -> String {
    let mut result = String::new();
    if value < 0 {
        result.push('⁻');
    }
    for character in value.unsigned_abs().to_string().chars() {
        result.push(match character {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!(),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantities_use_compact_engineering_notation() {
        assert_eq!(
            format_quantity(
                0.000_012_3,
                "m²/s",
                InspectNumberFormat::significant_digits(3),
            ),
            "12.3 × 10⁻⁶ m²/s",
        );
    }
}
