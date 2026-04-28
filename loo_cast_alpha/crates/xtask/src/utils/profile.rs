#[derive(Copy, Clone)]
pub enum Profile {
    Fastdev,
}

impl Profile {
    pub fn as_str(self) -> &'static str {
        "fastdev"
    }

    pub fn artifact_dir_name(self) -> &'static str {
        "fastdev"
    }
}
