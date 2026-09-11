use bevy::prelude::*;

use super::super::catalog::{
    PlaygroundCatalog,
    PlaygroundItemId,
};

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ItemView {
    pub item: Option<PlaygroundItemId>,
}

#[derive(Component)]
pub struct ItemViewLabel;

pub fn spawn_item_view(
    parent: &mut ChildSpawnerCommands,
    item: Option<PlaygroundItemId>,
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
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
            ));
        });
}

pub fn sync_item_views(
    catalog: Res<PlaygroundCatalog>,
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
