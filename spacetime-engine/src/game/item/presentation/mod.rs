//! Generic UI presentation of item definitions.

use bevy::prelude::*;

use crate::ui::UiTextStyle;

use super::{ItemCatalog, ItemId};

pub(super) fn configure(app: &mut App) {
    app.add_systems(PostUpdate, sync_item_views);
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ItemView {
    pub item: Option<ItemId>,
}

#[derive(Component)]
struct ItemViewLabel;

pub fn spawn_item_view(
    parent: &mut ChildSpawnerCommands,
    item: Option<ItemId>,
    text_style: &UiTextStyle,
) {
    parent
        .spawn((
            ItemView { item },
            Node {
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(px(4.0)),
                ..default()
            },
        ))
        .with_children(|view| {
            view.spawn((
                ItemViewLabel,
                Text::new(""),
                text_style.font(),
                text_style.color(),
                TextLayout::justify(Justify::Center),
            ));
        });
}

fn sync_item_views(
    catalog: Res<ItemCatalog>,
    views: Query<(&ItemView, &Children), Changed<ItemView>>,
    mut labels: Query<&mut Text, With<ItemViewLabel>>,
) {
    for (view, children) in &views {
        let label = view
            .item
            .and_then(|id| catalog.find(id))
            .map(|item| item.name)
            .unwrap_or("");

        for child in children.iter() {
            if let Ok(mut text) = labels.get_mut(child) {
                text.0 = label.to_owned();
            }
        }
    }
}
