use std::path::Path;
use std::process::Command;

pub fn git_user_name(root: &Path) -> Option<String> {
    let output = Command::new("git").arg("config").arg("user.name").current_dir(root).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let user_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if user_name.is_empty() { None } else { Some(user_name) }
}