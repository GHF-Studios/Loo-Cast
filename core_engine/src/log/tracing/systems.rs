use bevy::prelude::*;

use crate::log::{resources::LogRegistry, statics::{LOG_EVENT_BUFFER, SPAN_EVENT_BUFFER}};

pub(in super) fn flush_span_event_buffer_system(mut log_registry: ResMut<LogRegistry>) {
    while let Some(span_path) = SPAN_EVENT_BUFFER.pop() {
        log_registry.insert_without_log(&span_path);
    }
}
pub(in super) fn flush_log_event_buffer_system(mut log_registry: ResMut<LogRegistry>) {
    while let Some((log_id, log_entry, span_path,  module_path, physical_path)) = LOG_EVENT_BUFFER.pop() {
        log_registry.insert_log(log_id, log_entry, span_path, module_path, physical_path);
    }
}