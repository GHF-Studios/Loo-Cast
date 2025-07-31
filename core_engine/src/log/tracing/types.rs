use tracing::{
    span::Attributes,
    span::Id,
    Event,
};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::LookupSpan,
};

use crate::log::{statics::{LOG_EVENT_BUFFER, SPAN_EVENT_BUFFER}, tracing::functions::extract_span_identity};

use super::functions::extract_log_identity;

pub struct LogTreeTracingLayer;
impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        //if let Some(span_ref) = ctx.span(id) {
        //    let metadata = span_ref.metadata();
        //    let module_path = metadata.module_path();
        //}
        let span_path = extract_span_identity(&ctx);
        SPAN_EVENT_BUFFER.push(span_path);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let (
            log_id, 
            log_entry, 
            span_path, 
            module_path, 
            physical_path
        ) = extract_log_identity(event, &ctx);
        LOG_EVENT_BUFFER.push((log_id, log_entry, span_path, module_path, physical_path));
    }
}