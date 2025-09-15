use std::path::PathBuf;

use super::statics::start_time;

pub fn now_since_start_ns() -> u64 {
    start_time().elapsed().as_nanos() as u64
}

pub fn asset_root() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .expect("failed to get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();

    exe_dir.join("assets")
}