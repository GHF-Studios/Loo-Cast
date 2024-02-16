use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIObjectState {
    Disabled,
    Enabled(bevy::ecs::entity::Entity),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIObjectEnableError {
    AlreadyEnabled,
    ParentDisabled,
}

impl fmt::Display for UIObjectEnableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIObjectEnableError::AlreadyEnabled => write!(f, "The UI object is already enabled."),
            UIObjectEnableError::ParentDisabled => write!(f, "The parent UI object is disabled."),
        }
    }
}

impl Error for UIObjectEnableError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIObjectDisableError {
    AlreadyDisabled,
    ParentEnabled,
}

impl fmt::Display for UIObjectDisableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIObjectDisableError::AlreadyDisabled => {
                write!(f, "The UI object is already disabled.")
            }
            UIObjectDisableError::ParentEnabled => write!(f, "The parent UI object is enabled."),
        }
    }
}

impl Error for UIObjectDisableError {}

// TODO: IMPLEMENT PROPERLY
pub trait UIObject {
    fn get_ui_object_state(&self) -> UIObjectState;
    fn set_ui_object_state(&mut self, state: UIObjectState);

    fn on_enable(&mut self) -> Result<(), UIObjectEnableError>;
    fn on_disable(&mut self) -> Result<(), UIObjectDisableError>;

    fn on_focus(&self);
    fn on_unfocus(&self);
}