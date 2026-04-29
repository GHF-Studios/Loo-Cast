use crate::{LINUX_RELEASE_TARGET, WINDOWS_RELEASE_TARGET};

#[derive(Copy, Clone)]
pub enum BuildTarget {
    Host,
    LinuxRelease,
    WindowsRelease,
}
impl BuildTarget {
    pub fn triple(self) -> Option<&'static str> {
        match self {
            BuildTarget::Host => None,
            BuildTarget::LinuxRelease => Some(LINUX_RELEASE_TARGET),
            BuildTarget::WindowsRelease => Some(WINDOWS_RELEASE_TARGET),
        }
    }

    pub fn is_windows(self) -> bool {
        match self {
            BuildTarget::Host => cfg!(target_os = "windows"),
            BuildTarget::LinuxRelease => false,
            BuildTarget::WindowsRelease => true,
        }
    }
}
