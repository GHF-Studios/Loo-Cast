#[derive(Copy, Clone)]
pub enum BuildTarget {
    Host,
}
impl BuildTarget {
    pub fn triple(self) -> Option<&'static str> {
        None
    }

    pub fn is_windows(self) -> bool {
        cfg!(target_os = "windows")
    }
}
