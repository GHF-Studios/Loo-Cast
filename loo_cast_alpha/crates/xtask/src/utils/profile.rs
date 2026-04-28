#[derive(Copy, Clone)]
pub enum Profile {
    Dev,
    Fastdev,
    Release,
}

impl Profile {
    pub fn as_str(self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::Fastdev => "fastdev",
            Profile::Release => "release",
        }
    }

    pub fn artifact_dir_name(self) -> &'static str {
        match self {
            Profile::Dev => "debug",
            Profile::Fastdev => "fastdev",
            Profile::Release => "release",
        }
    }
}