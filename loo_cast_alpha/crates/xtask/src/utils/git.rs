use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn git_user_name(root: &Path) -> Option<String> {
    let output = Command::new("git").arg("config").arg("user.name").current_dir(root).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let user_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if user_name.is_empty() { None } else { Some(user_name) }
}

pub fn find_git_dir(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        let git_path = ancestor.join(".git");
        if git_path.is_dir() {
            return Some(git_path);
        }
        if git_path.is_file()
            && let Some(git_dir) = parse_gitdir_file(&git_path)
        {
            return Some(git_dir);
        }
    }
    None
}

fn parse_gitdir_file(git_file_path: &Path) -> Option<PathBuf> {
    let content = fs::read_to_string(git_file_path).ok()?;
    let git_dir_raw = content.lines().find_map(|line| line.strip_prefix("gitdir:"))?.trim();
    if git_dir_raw.is_empty() {
        return None;
    }
    let git_dir_path = PathBuf::from(git_dir_raw);
    if git_dir_path.is_absolute() {
        Some(git_dir_path)
    } else {
        Some(git_file_path.parent()?.join(git_dir_path))
    }
}
