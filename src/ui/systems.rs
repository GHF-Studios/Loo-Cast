use crate::ui::events::*;
use crate::ui::resources::FocusManager;

use bevy::prelude::*;

pub fn handle_gained_focus_event(
    mut focus_manager: ResMut<FocusManager>,
    mut gained_focus_event_reader: EventReader<GainedFocus>,
    mut lost_focus_event_writer: EventWriter<LostFocus>,
) {
    if let Some(gained_focus_event) = gained_focus_event_reader.iter().last() {
        if let Some(old_focus) = focus_manager.focus {
            lost_focus_event_writer.send(LostFocus { entity: old_focus });
        }
        focus_manager.focus = Some(gained_focus_event.entity);
    }
}
