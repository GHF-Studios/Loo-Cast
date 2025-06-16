use tracing::{Event, Subscriber};
use tracing_subscriber::{
    fmt::{FormatEvent, FormatFields, FmtContext, format::Writer, time::FormatTime},
    registry::LookupSpan,
};
use std::fmt;

use crate::config::statics::CONFIG;
use crate::statics::START_TIME;

pub struct ShortTime;
impl FormatTime for ShortTime {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        let elapsed = START_TIME.elapsed();
        let millis = elapsed.subsec_millis();
        let secs = elapsed.as_secs() % 60;
        let mins = (elapsed.as_secs() / 60) % 60;
        write!(w, "T+ {:02}m:{:02}s.{:03}ms", mins, secs, millis)
    }
}
