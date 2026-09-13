//! Focused, screen-space developer Inspector.
//!
//! World-space labels are intentionally terse spatial annotations. Rich,
//! scientific state belongs here where it can be aligned, bounded and read
//! without competing with nearby objects.

use std::sync::{RwLock, RwLockReadGuard};

use bevy::{prelude::*, text::FontSize};

use super::{
    DebugArtifact, DebugContext, DebugControls, DebugId, ObservabilitySet,
    format_quantity, scientific_unit,
};

#[derive(Debug, Clone)]
pub struct DebugInspectorRow {
    pub label: String,
    pub value: String,
}

impl DebugInspectorRow {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugInspectorSection {
    pub id: DebugId,
    pub title: String,
    pub order: i32,
    pub rows: Vec<DebugInspectorRow>,
}

impl DebugInspectorSection {
    pub fn new(id: DebugId, title: impl Into<String>, order: i32) -> Self {
        Self {
            id,
            title: title.into(),
            order,
            rows: Vec::new(),
        }
    }

    pub fn row(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.rows.push(DebugInspectorRow::new(label, value));
        self
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugInspector {
    sections: RwLock<Vec<DebugInspectorSection>>,
}

impl DebugInspector {
    pub fn submit(&self, section: DebugInspectorSection) {
        let mut sections = self.sections.write().expect("debug inspector lock poisoned");
        assert!(
            !sections.iter().any(|existing| existing.id == section.id),
            "duplicate debug inspector section id {}",
            section.id
        );
        sections.push(section);
    }

    fn clear(&self) {
        self.sections
            .write()
            .expect("debug inspector lock poisoned")
            .clear();
    }

    fn read(&self) -> RwLockReadGuard<'_, Vec<DebugInspectorSection>> {
        self.sections.read().expect("debug inspector lock poisoned")
    }
}

#[derive(Component)]
struct DebugInspectorRoot;

pub(super) fn configure(app: &mut App) {
    // Stage 2 migration bridge: legacy domain collectors may still submit
    // sections here, but presentation has moved to `devtools::ui`. Keep clearing
    // the sink so those collectors cannot accumulate duplicate section IDs.
    app.init_resource::<DebugInspector>().add_systems(
        PostUpdate,
        clear_debug_inspector.in_set(ObservabilitySet::Prepare),
    );
}

fn clear_debug_inspector(inspector: Res<DebugInspector>) {
    inspector.clear();
}

fn rebuild_inspector(
    mut commands: Commands,
    controls: Res<DebugControls>,
    context: Res<DebugContext>,
    inspector: Res<DebugInspector>,
    names: Query<&Name>,
    roots: Query<Entity, With<DebugInspectorRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }

    if !controls.master_enabled() {
        return;
    }

    let Some(selection) = context.selection else {
        return;
    };

    let semantic_name = names
        .get(selection.semantic_entity)
        .map(|name| name.as_str())
        .unwrap_or("Unnamed entity");
    let focused_name = names
        .get(selection.entity)
        .map(|name| name.as_str())
        .unwrap_or("Unnamed spatial entity");

    let inspector_sections = inspector.read();
    let mut sections = inspector_sections.iter().collect::<Vec<_>>();
    sections.sort_by(|a, b| {
        a.order
            .cmp(&b.order)
            .then_with(|| a.title.cmp(&b.title))
    });

    commands
        .spawn((
            Name::new("Debug Inspector"),
            DebugArtifact,
            DebugInspectorRoot,
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                left: px(16),
                width: px(430),
                max_height: percent(92.0),
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(px(12)),
                row_gap: px(6),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.018, 0.022, 0.034, 0.965)),
            GlobalZIndex(1_900),
        ))
        .with_children(|parent| {
            spawn_text(parent, "INSPECTOR", 20.0, Color::WHITE);
            spawn_text(
                parent,
                semantic_name,
                15.0,
                Color::srgb(0.82, 0.90, 1.0),
            );

            spawn_row(
                parent,
                "Semantic",
                format!("{:?}", selection.semantic_entity),
            );
            if selection.entity != selection.semantic_entity {
                spawn_row(parent, "Focused", focused_name);
                spawn_row(parent, "Spatial entity", format!("{:?}", selection.entity));
            }
            spawn_row(
                parent,
                "Distance",
                format_quantity(
                    selection.distance_meters,
                    scientific_unit::METER,
                    4,
                ),
            );

            for section in sections {
                parent.spawn((
                    DebugArtifact,
                    Node {
                        width: percent(100.0),
                        height: px(1),
                        margin: UiRect::vertical(px(4)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.32, 0.38, 0.50, 0.55)),
                ));
                spawn_text(
                    parent,
                    section.title.clone(),
                    13.0,
                    Color::srgb(0.72, 0.80, 0.94),
                );
                for row in &section.rows {
                    spawn_row(parent, row.label.clone(), row.value.clone());
                }
            }
        });
}

fn spawn_row(
    parent: &mut ChildSpawnerCommands,
    label: impl Into<String>,
    value: impl Into<String>,
) {
    parent
        .spawn((
            DebugArtifact,
            Node {
                width: percent(100.0),
                column_gap: px(10),
                align_items: AlignItems::Start,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    DebugArtifact,
                    Node {
                        width: px(145),
                        flex_shrink: 0.0,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_text(
                        parent,
                        label,
                        11.0,
                        Color::srgb(0.52, 0.60, 0.72),
                    );
                });

            parent
                .spawn((
                    DebugArtifact,
                    Node {
                        flex_grow: 1.0,
                        min_width: px(0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_text(parent, value, 11.0, Color::WHITE);
                });
        });
}

fn spawn_text(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    size: f32,
    color: Color,
) {
    parent.spawn((
        DebugArtifact,
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    ));
}
