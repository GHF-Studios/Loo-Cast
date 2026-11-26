#![cfg_attr(windows, windows_subsystem = "windows")]

//! Steam entry point for the Loo-Cast Packagepack.

mod steam_tools;

fn main() {
    match steam_tools::launch_mode(std::env::args_os().skip(1)) {
        steam_tools::LaunchMode::Game => spacetime_engine::run(loo_cast_game::install),
        steam_tools::LaunchMode::VaporTools => {
            if let Err(error) = steam_tools::open_vapor_tools() {
                steam_tools::show_install_help(&error);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::steam_tools::{LaunchMode, launch_mode};
    use std::ffi::OsString;

    #[test]
    fn no_switch_starts_the_game() {
        assert_eq!(launch_mode([]), LaunchMode::Game);
    }

    #[test]
    fn tools_switch_opens_the_vapor_chooser() {
        assert_eq!(
            launch_mode([OsString::from("--vapor-tools")]),
            LaunchMode::VaporTools
        );
    }

    #[test]
    fn unrelated_game_arguments_do_not_change_the_launch_mode() {
        assert_eq!(
            launch_mode([OsString::from("--windowed")]),
            LaunchMode::Game
        );
    }
}
