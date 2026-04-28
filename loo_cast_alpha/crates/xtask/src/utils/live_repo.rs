use std::path::{Path, PathBuf};

pub fn find_live_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|path| {
            path.join(".git").exists()
                && path.join("Cargo.toml").is_file()
                && path.join("crates").is_dir()
                && path.join("crates").join("xtask").join("Cargo.toml").is_file()
        })
        .map(Path::to_path_buf)
}
