use crate::statics::START_TIME;

pub fn now_since_start_ns() -> u64 {
    START_TIME.elapsed().as_nanos() as u64
}