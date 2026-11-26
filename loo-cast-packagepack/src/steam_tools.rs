use std::{env, ffi::OsString, path::PathBuf, process::Command};

const TOOLS_SWITCH: &str = "--vapor-tools";
const DOWNLOADS_URL: &str = "https://vapor.ghf-studios.site/downloads";
const HOME_MARKER: &str = "Vapor-Home.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LaunchMode {
    Game,
    VaporTools,
}

pub(super) fn launch_mode(arguments: impl IntoIterator<Item = OsString>) -> LaunchMode {
    if arguments
        .into_iter()
        .any(|argument| argument == TOOLS_SWITCH)
    {
        LaunchMode::VaporTools
    } else {
        LaunchMode::Game
    }
}

pub(super) fn open_vapor_tools() -> Result<(), String> {
    let home = vapor_home().ok_or_else(|| "Vapor is not installed".to_owned())?;
    if !home.join(HOME_MARKER).is_file() {
        return Err("Vapor's home is missing its installation marker".to_owned());
    }

    let executable = home
        .join("bin")
        .join(runtime_target())
        .join(gui_binary_name());
    if !executable.is_file() {
        return Err(format!(
            "Vapor is installed, but its tools selector is missing at {}",
            executable.display()
        ));
    }

    Command::new(&executable)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("could not open Vapor's tools selector: {error}"))
}

fn vapor_home() -> Option<PathBuf> {
    env::var_os("VAPOR_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| user_home().map(|home| home.join("Vapor")))
}

fn user_home() -> Option<PathBuf> {
    #[cfg(windows)]
    let home = env::var_os("USERPROFILE").or_else(|| env::var_os("HOME"));
    #[cfg(not(windows))]
    let home = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE"));
    home.map(PathBuf::from)
}

#[cfg(windows)]
fn runtime_target() -> &'static str {
    "x86_64-pc-windows-gnullvm"
}

#[cfg(not(windows))]
fn runtime_target() -> &'static str {
    "x86_64-unknown-linux-gnu"
}

#[cfg(windows)]
fn gui_binary_name() -> &'static str {
    "vapor-gui.exe"
}

#[cfg(not(windows))]
fn gui_binary_name() -> &'static str {
    "vapor-gui"
}

pub(super) fn show_install_help(reason: &str) {
    let message = format!(
        "Vapor's tools could not be opened. {reason}\n\nInstall Vapor independently, then choose Tools in Steam again.\n{DOWNLOADS_URL}"
    );

    #[cfg(windows)]
    show_windows_message(&message);
    #[cfg(not(windows))]
    {
        eprintln!("{message}");
        let _ = Command::new("xdg-open").arg(DOWNLOADS_URL).spawn();
    }
}

#[cfg(windows)]
fn show_windows_message(message: &str) {
    #[link(name = "user32")]
    unsafe extern "system" {
        fn MessageBoxW(
            window: *mut core::ffi::c_void,
            text: *const u16,
            caption: *const u16,
            kind: u32,
        ) -> i32;
    }

    let text = wide_nul(message);
    let caption = wide_nul("Vapor tools unavailable");
    // SAFETY: both pointers reference NUL-terminated buffers alive for this call.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            0x0000_0040,
        );
    }
    let _ = Command::new("explorer.exe").arg(DOWNLOADS_URL).spawn();
}

#[cfg(windows)]
fn wide_nul(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
