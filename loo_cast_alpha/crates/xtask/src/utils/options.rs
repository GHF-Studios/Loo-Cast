#[derive(Clone, Copy, Debug)]
pub struct XtaskOptions {
    pub clean_sdk: bool,
    pub use_vendored_toolchain: bool,
    pub contribute_apply: bool,
    pub contribute_push: bool,
}

impl Default for XtaskOptions {
    fn default() -> Self {
        Self {
            clean_sdk: false,
            use_vendored_toolchain: true,
            contribute_apply: false,
            contribute_push: true,
        }
    }
}
