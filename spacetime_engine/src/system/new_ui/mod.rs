use core::panic;
use std::{any::TypeId, collections::HashMap, collections::HashSet};
use std::fmt;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIObjectState {
    Enabled,
    Disabled
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIObjectDisableError {
    AlreadyDisabled,
    ParentEnabled,
}

impl fmt::Display for UIObjectDisableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIObjectDisableError::AlreadyDisabled => write!(f, "The UI object is already disabled."),
            UIObjectDisableError::ParentEnabled => write!(f, "The parent UI object is enabled."),
        }
    }
}

impl Error for UIObjectDisableError {}

/// IMPLEMENT PROPERLY
pub trait UIObject {
    fn get_ui_object_state(&self) -> UIObjectState;
    
    fn on_enable(&mut self) -> Result<(), UIObjectEnableError>;
    fn on_disable(&mut self) -> Result<(), UIObjectDisableError>;

    fn on_focus(&self);
    fn on_unfocus(&self);
}

/// IMPLEMENT PROPERLY
pub trait UIEvent {
}

/// IMPLEMENT PROPERLY
pub trait UIEventHandler {
}

type UIElementID = usize;

pub trait UIElement: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UIElementID>;
    fn set_id(&mut self, element_id: Option<UIElementID>);
    
    fn get_parent(&self) -> Option<UIContainerID>;
    fn set_parent(&mut self, container_id: Option<UIContainerID>);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerParentType {
    Window,
    Container,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerChildType {
    Element,
    Container,
}

type UIContainerID = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerInsertElementError {
    AlreadyInserted,
}

impl fmt::Display for UIContainerInsertElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerInsertElementError::AlreadyInserted => write!(f, "The element is already inserted."),
        }
    }
}

impl Error for UIContainerInsertElementError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerInsertContainerError {
    AlreadyInserted,
}

impl fmt::Display for UIContainerInsertContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerInsertContainerError::AlreadyInserted => write!(f, "The container is already inserted."),
        }
    }
}

impl Error for UIContainerInsertContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerRemoveElementError {
    AlreadyRemoved,
}

impl fmt::Display for UIContainerRemoveElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerRemoveElementError::AlreadyRemoved => write!(f, "The element is already removed."),
        }
    }
}

impl Error for UIContainerRemoveElementError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIContainerRemoveContainerError {
    AlreadyRemoved,
}

impl fmt::Display for UIContainerRemoveContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerRemoveContainerError::AlreadyRemoved => write!(f, "The container is already removed."),
        }
    }
}

impl Error for UIContainerRemoveContainerError {}

pub trait UIContainer: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UIContainerID>;
    fn set_id(&mut self, container_id: Option<UIContainerID>);

    fn get_parent(&self) -> Option<(UIContainerParentType, usize)>;
    fn set_parent_container(&mut self, container_id: Option<UIContainerID>);
    fn set_parent_window(&mut self, window_id: Option<UIWindowID>);

    fn insert_element(&mut self, element_id: UIElementID) -> Result<(), UIContainerInsertElementError>;
    fn insert_container(&mut self, container_id: UIContainerID) -> Result<(), UIContainerInsertContainerError>;

    fn remove_element(&mut self, element_id: UIElementID) -> Result<(), UIContainerRemoveElementError>;
    fn remove_container(&mut self, container_id: UIContainerID) -> Result<(), UIContainerRemoveContainerError>;

    fn get_elements(&self) -> &HashSet<UIElementID>;
    fn get_containers(&self) -> &HashSet<UIContainerID>;

    fn contains_element(&self, element_id: UIElementID) -> bool;
    fn contains_container(&self, container_id: UIContainerID) -> bool;
}

type UIWindowID = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIWindowInsertContainerError {
    AlreadyInserted,
}

impl fmt::Display for UIWindowInsertContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIWindowInsertContainerError::AlreadyInserted => write!(f, "The container is already inserted."),
        }
    }
}

impl Error for UIWindowInsertContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIWindowRemoveContainerError {
    AlreadyRemoved,
}

impl fmt::Display for UIWindowRemoveContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIWindowRemoveContainerError::AlreadyRemoved => write!(f, "The container is already removed."),
        }
    }
}

impl Error for UIWindowRemoveContainerError {}

pub trait UIWindow: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UIWindowID>;
    fn set_id(&mut self, window_id: Option<UIWindowID>);

    fn get_parent(&self) -> Option<UICanvasID>;
    fn set_parent(&mut self, canvas_id: Option<UICanvasID>);
    
    fn get_size(&self) -> (f32, f32);
    fn get_position(&self) -> (f32, f32);

    fn set_size(&mut self, width: f32, height: f32);
    fn set_position(&mut self, x: f32, y: f32);

    fn insert_container(&mut self, container_id: UIContainerID) -> Result<(), UIWindowInsertContainerError>;
    fn remove_container(&mut self, container_id: UIContainerID) -> Result<(), UIWindowRemoveContainerError>;

    fn get_containers(&self) -> &HashSet<UIContainerID>;

    fn contains_container(&self, container_id: UIContainerID) -> bool;
}

type UICanvasID = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UICanvasRenderingContext {
    ScreenSpace,
    WorldSpace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UICanvasInsertWindowError {
    AlreadyInserted,
}

impl fmt::Display for UICanvasInsertWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UICanvasInsertWindowError::AlreadyInserted => write!(f, "The window is already inserted."),
        }
    }
}

impl Error for UICanvasInsertWindowError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UICanvasRemoveWindowError {
    AlreadyRemoved,
}

impl fmt::Display for UICanvasRemoveWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UICanvasRemoveWindowError::AlreadyRemoved => write!(f, "The window is already removed."),
        }
    }
}

impl Error for UICanvasRemoveWindowError {}

pub trait UICanvas: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UICanvasID>;
    fn set_id(&mut self, canvas_id: Option<UICanvasID>);

    fn get_parent(&self) -> Option<UISceneID>;
    fn set_parent(&mut self, scene_id: Option<UISceneID>);

    fn insert_window(&mut self, window_id: UIWindowID) -> Result<(), UICanvasInsertWindowError>;
    fn remove_window(&mut self, window_id: UIWindowID) -> Result<(), UICanvasRemoveWindowError>;

    fn get_windows(&self) -> &HashSet<UIWindowID>;

    fn contains_window(&self, window_id: UIWindowID) -> bool;
}

pub type UISceneID = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UISceneInsertCanvasError {
    AlreadyInserted,
}

impl fmt::Display for UISceneInsertCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UISceneInsertCanvasError::AlreadyInserted => write!(f, "The canvas is already inserted."),
        }
    }
}

impl Error for UISceneInsertCanvasError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UISceneRemoveCanvasError {
    AlreadyRemoved,
}

impl fmt::Display for UISceneRemoveCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UISceneRemoveCanvasError::AlreadyRemoved => write!(f, "The canvas is already removed."),
        }
    }
}

impl Error for UISceneRemoveCanvasError {}

pub trait UIScene: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UISceneID>;
    fn set_id(&mut self, scene_id: Option<UISceneID>);

    fn insert_canvas(&mut self, canvas_id: UICanvasID) -> Result<(), UISceneInsertCanvasError>;
    fn remove_canvas(&mut self, canvas_id: UICanvasID) -> Result<(), UISceneRemoveCanvasError>;

    fn get_canvases(&self) -> &HashSet<UICanvasID>;

    fn contains_canvas(&self, canvas_id: UICanvasID) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterSceneTypeError {
    AlreadyRegistered,
}

impl fmt::Display for UIManagerRegisterSceneTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterSceneTypeError::AlreadyRegistered => write!(f, "The scene type is already registered."),
        }
    }
}

impl Error for UIManagerRegisterSceneTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterCanvasTypeError {
    AlreadyRegistered,
}

impl fmt::Display for UIManagerRegisterCanvasTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterCanvasTypeError::AlreadyRegistered => write!(f, "The canvas type is already registered."),
        }
    }
}

impl Error for UIManagerRegisterCanvasTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterWindowTypeError {
    AlreadyRegistered,
}

impl fmt::Display for UIManagerRegisterWindowTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterWindowTypeError::AlreadyRegistered => write!(f, "The window type is already registered."),
        }
    }
}

impl Error for UIManagerRegisterWindowTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterContainerTypeError {
    AlreadyRegistered,
}

impl fmt::Display for UIManagerRegisterContainerTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterContainerTypeError::AlreadyRegistered => write!(f, "The container type is already registered."),
        }
    }
}

impl Error for UIManagerRegisterContainerTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterElementTypeError {
    AlreadyRegistered,
}

impl fmt::Display for UIManagerRegisterElementTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterElementTypeError::AlreadyRegistered => write!(f, "The element type is already registered."),
        }
    }
}

impl Error for UIManagerRegisterElementTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterSceneTypeError {
    AlreadyUnregistered,
    InstancesStillRegistered,
}

impl fmt::Display for UIManagerUnregisterSceneTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterSceneTypeError::AlreadyUnregistered => write!(f, "The scene type is already unregistered."),
            UIManagerUnregisterSceneTypeError::InstancesStillRegistered => write!(f, "There are still instances of the scene type registered."),
        }
    }
}

impl Error for UIManagerUnregisterSceneTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterCanvasTypeError {
    AlreadyUnregistered,
    InstancesStillRegistered,
}

impl fmt::Display for UIManagerUnregisterCanvasTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterCanvasTypeError::AlreadyUnregistered => write!(f, "The canvas type is already unregistered."),
            UIManagerUnregisterCanvasTypeError::InstancesStillRegistered => write!(f, "There are still instances of the canvas type registered."),
        }
    }
}

impl Error for UIManagerUnregisterCanvasTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterWindowTypeError {
    AlreadyUnregistered,
    InstancesStillRegistered,
}

impl fmt::Display for UIManagerUnregisterWindowTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterWindowTypeError::AlreadyUnregistered => write!(f, "The window type is already unregistered."),
            UIManagerUnregisterWindowTypeError::InstancesStillRegistered => write!(f, "There are still instances of the window type registered."),
        }
    }
}

impl Error for UIManagerUnregisterWindowTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterContainerTypeError {
    AlreadyUnregistered,
    InstancesStillRegistered,
}

impl fmt::Display for UIManagerUnregisterContainerTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterContainerTypeError::AlreadyUnregistered => write!(f, "The container type is already unregistered."),
            UIManagerUnregisterContainerTypeError::InstancesStillRegistered => write!(f, "There are still instances of the container type registered."),
        }
    }
}

impl Error for UIManagerUnregisterContainerTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterElementTypeError {
    AlreadyUnregistered,
    InstancesStillRegistered,
}

impl fmt::Display for UIManagerUnregisterElementTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterElementTypeError::AlreadyUnregistered => write!(f, "The element type is already unregistered."),
            UIManagerUnregisterElementTypeError::InstancesStillRegistered => write!(f, "There are still instances of the element type registered."),
        }
    }
}

impl Error for UIManagerUnregisterElementTypeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterSceneError {
    TypeNotRegistered,
    AlreadyRegistered,
    GetUnusedSceneIDError(UIManagerGetUnusedSceneIDError),
}

impl fmt::Display for UIManagerRegisterSceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterSceneError::TypeNotRegistered => write!(f, "The scene type is not registered."),
            UIManagerRegisterSceneError::AlreadyRegistered => write!(f, "The scene is already registered."),
            UIManagerRegisterSceneError::GetUnusedSceneIDError(e) => write!(f, "An error occurred while getting an unused scene ID: {}", e),
        }
    }
}

impl Error for UIManagerRegisterSceneError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterCanvasError {
    TypeNotRegistered,
    AlreadyRegistered,
    NoParentScene,
    ParentSceneNotRegistered,
    GetUnusedCanvasIDError(UIManagerGetUnusedCanvasIDError),
}

impl fmt::Display for UIManagerRegisterCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterCanvasError::TypeNotRegistered => write!(f, "The canvas type is not registered."),
            UIManagerRegisterCanvasError::AlreadyRegistered => write!(f, "The canvas is already registered."),
            UIManagerRegisterCanvasError::NoParentScene => write!(f, "The canvas has no parent scene."),
            UIManagerRegisterCanvasError::ParentSceneNotRegistered => write!(f, "The parent scene is not registered."),	
            UIManagerRegisterCanvasError::GetUnusedCanvasIDError(e) => write!(f, "An error occurred while getting an unused canvas ID: {}", e),
        }
    }
}

impl Error for UIManagerRegisterCanvasError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterWindowError {
    TypeNotRegistered,
    AlreadyRegistered,
    NoParentCanvas,
    ParentCanvasNotRegistered,
    GetUnusedWindowIDError(UIManagerGetUnusedWindowIDError),
}

impl fmt::Display for UIManagerRegisterWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterWindowError::TypeNotRegistered => write!(f, "The window type is not registered."),
            UIManagerRegisterWindowError::AlreadyRegistered => write!(f, "The window is already registered."),
            UIManagerRegisterWindowError::NoParentCanvas => write!(f, "The window has no parent canvas."),
            UIManagerRegisterWindowError::ParentCanvasNotRegistered => write!(f, "The parent canvas is not registered."),
            UIManagerRegisterWindowError::GetUnusedWindowIDError(e) => write!(f, "An error occurred while getting an unused window ID: {}", e),
        }
    }
}

impl Error for UIManagerRegisterWindowError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterContainerError {
    TypeNotRegistered,
    AlreadyRegistered,
    NoParent,
    ParentWindowNotRegistered,
    ParentContainerNotRegistered,
    GetUnusedContainerIDError(UIManagerGetUnusedContainerIDError),
}

impl fmt::Display for UIManagerRegisterContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterContainerError::TypeNotRegistered => write!(f, "The container type is not registered."),
            UIManagerRegisterContainerError::AlreadyRegistered => write!(f, "The container is already registered."),
            UIManagerRegisterContainerError::NoParent => write!(f, "The container has no parent window or container."),
            UIManagerRegisterContainerError::ParentWindowNotRegistered => write!(f, "The parent window is not registered."),
            UIManagerRegisterContainerError::ParentContainerNotRegistered => write!(f, "The parent container is not registered."),
            UIManagerRegisterContainerError::GetUnusedContainerIDError(e) => write!(f, "An error occurred while getting an unused container ID: {}", e),
        }
    }
}

impl Error for UIManagerRegisterContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterElementError {
    TypeNotRegistered,
    AlreadyRegistered,
    NoParentContainer,
    ParentContainerNotRegistered,
    GetUnusedElementIDError(UIManagerGetUnusedElementIDError),
}

impl fmt::Display for UIManagerRegisterElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRegisterElementError::TypeNotRegistered => write!(f, "The element type is not registered."),
            UIManagerRegisterElementError::AlreadyRegistered => write!(f, "The element is already registered."),
            UIManagerRegisterElementError::NoParentContainer => write!(f, "The element has no parent container."),
            UIManagerRegisterElementError::ParentContainerNotRegistered => write!(f, "The parent container is not registered."),
            UIManagerRegisterElementError::GetUnusedElementIDError(e) => write!(f, "An error occurred while getting an unused element ID: {}", e),
        }
    }
}

impl Error for UIManagerRegisterElementError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterSceneError {
    AlreadyUnregistered,
    UnregisterChildCanvasErrors(Box<[(UICanvasID, UIManagerUnregisterCanvasError)]>),
    RecycleSceneIDError(UIManagerRecycleSceneIDError),
}

impl fmt::Display for UIManagerUnregisterSceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterSceneError::AlreadyUnregistered => write!(f, "The scene is already unregistered."),
            UIManagerUnregisterSceneError::UnregisterChildCanvasErrors(errors) => {
                if errors.is_empty() {
                    write!(f, "An unknown error occurred while unregistering the child canvases.")
                } else {
                    write!(f, "The following errors occurred while unregistering the child canvases:")?;

                    for (canvas_id, error) in errors.iter() {
                        write!(f, "\nCanvas ID: {}\nError: {}", canvas_id, error)?;
                    }

                    Ok(())
                }
            },
            UIManagerUnregisterSceneError::RecycleSceneIDError(e) => write!(f, "An error occurred while recycling the scene ID: {}", e),
        }
    }
}

impl Error for UIManagerUnregisterSceneError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterCanvasError {
    AlreadyUnregistered,
    UnregisterChildWindowErrors(Box<[(UIWindowID, UIManagerUnregisterWindowError)]>),
    RecycleCanvasIDError(UIManagerRecycleCanvasIDError),
}

impl fmt::Display for UIManagerUnregisterCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterCanvasError::AlreadyUnregistered => write!(f, "The canvas is already unregistered."),
            UIManagerUnregisterCanvasError::UnregisterChildWindowErrors(errors) => {
                if errors.is_empty() {
                    write!(f, "An unknown error occurred while unregistering the child windows.")
                } else {
                    write!(f, "The following errors occurred while unregistering the child windows:")?;

                    for (window_id, error) in errors.iter() {
                        write!(f, "\nWindow ID: {}\nError: {}", window_id, error)?;
                    }

                    Ok(())
                }
            },
            UIManagerUnregisterCanvasError::RecycleCanvasIDError(e) => write!(f, "An error occurred while recycling the canvas ID: {}", e),
        }
    }
}

impl Error for UIManagerUnregisterCanvasError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterWindowError {
    AlreadyUnregistered,
    UnregisterChildContainerErrors(Box<[(UIContainerID, UIManagerUnregisterContainerError)]>),
    RecycleWindowIDError(UIManagerRecycleWindowIDError),
}

impl fmt::Display for UIManagerUnregisterWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterWindowError::AlreadyUnregistered => write!(f, "The window is already unregistered."),
            UIManagerUnregisterWindowError::UnregisterChildContainerErrors(errors) => {
                if errors.is_empty() {
                    write!(f, "An unknown error occurred while unregistering the child containers.")
                } else {
                    write!(f, "The following errors occurred while unregistering the child containers:")?;

                    for (container_id, error) in errors.iter() {
                        write!(f, "\nContainer ID: {}\nError: {}", container_id, error)?;
                    }

                    Ok(())
                }
            },
            UIManagerUnregisterWindowError::RecycleWindowIDError(e) => write!(f, "An error occurred while recycling the window ID: {}", e),
        }
    }
}

impl Error for UIManagerUnregisterWindowError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterContainerError {
    AlreadyUnregistered,
    UnregisterChildElementErrors(Box<[(UIElementID, UIManagerUnregisterElementError)]>),
    RecycleContainerIDError(UIManagerRecycleContainerIDError),
}

impl fmt::Display for UIManagerUnregisterContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterContainerError::AlreadyUnregistered => write!(f, "The container is already unregistered."),
            UIManagerUnregisterContainerError::UnregisterChildElementErrors(errors) => {
                if errors.is_empty() {
                    write!(f, "An unknown error occurred while unregistering the child elements.")
                } else {
                    write!(f, "The following errors occurred while unregistering the child elements:")?;

                    for (element_id, error) in errors.iter() {
                        write!(f, "\nElement ID: {}\nError: {}", element_id, error)?;
                    }

                    Ok(())
                }
            },
            UIManagerUnregisterContainerError::RecycleContainerIDError(e) => write!(f, "An error occurred while recycling the container ID: {}", e),
        }
    }
}

impl Error for UIManagerUnregisterContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterElementError {
    AlreadyUnregistered,
    RecycleElementIDError(UIManagerRecycleElementIDError),
}

impl fmt::Display for UIManagerUnregisterElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnregisterElementError::AlreadyUnregistered => write!(f, "The element is already unregistered."),
            UIManagerUnregisterElementError::RecycleElementIDError(e) => write!(f, "An error occurred while recycling the element ID: {}", e),
        }
    }
}

impl Error for UIManagerUnregisterElementError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusSceneError {
    NotRegistered,
    UnfocusFocusedSceneError(UIManagerUnfocusSceneError),
}

impl fmt::Display for UIManagerFocusSceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerFocusSceneError::NotRegistered => write!(f, "The scene is not registered."),
            UIManagerFocusSceneError::UnfocusFocusedSceneError(e) => write!(f, "An error occurred while unfocusing the currently focused scene: {}", e),
        }
    }
}

impl Error for UIManagerFocusSceneError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusCanvasError {
    NotRegistered,
    UnfocusFocusedCanvasError(UIManagerUnfocusCanvasError),
}

impl fmt::Display for UIManagerFocusCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerFocusCanvasError::NotRegistered => write!(f, "The canvas is not registered."),
            UIManagerFocusCanvasError::UnfocusFocusedCanvasError(e) => write!(f, "An error occurred while unfocusing the currently focused canvas: {}", e),
        }
    }
}

impl Error for UIManagerFocusCanvasError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusWindowError {
    NotRegistered,
    UnfocusFocusedWindowError(UIManagerUnfocusWindowError),
}

impl fmt::Display for UIManagerFocusWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerFocusWindowError::NotRegistered => write!(f, "The window is not registered."),
            UIManagerFocusWindowError::UnfocusFocusedWindowError(e) => write!(f, "An error occurred while unfocusing the currently focused window: {}", e),
        }
    }
}

impl Error for UIManagerFocusWindowError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusContainerError {
    NotRegistered,
    UnfocusFocusedContainerError(UIManagerUnfocusContainerError),
}

impl fmt::Display for UIManagerFocusContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerFocusContainerError::NotRegistered => write!(f, "The container is not registered."),
            UIManagerFocusContainerError::UnfocusFocusedContainerError(e) => write!(f, "An error occurred while unfocusing the currently focused container: {}", e),
        }
    }
}

impl Error for UIManagerFocusContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusElementError {
    NotRegistered,
    UnfocusFocusedElementError(UIManagerUnfocusElementError),
}

impl fmt::Display for UIManagerFocusElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerFocusElementError::NotRegistered => write!(f, "The element is not registered."),
            UIManagerFocusElementError::UnfocusFocusedElementError(e) => write!(f, "An error occurred while unfocusing the currently focused element: {}", e),
        }
    }
}

impl Error for UIManagerFocusElementError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusSceneError {
    NotRegistered,
    UnfocusFocusedCanvasError(UIManagerUnfocusCanvasError),
}

impl fmt::Display for UIManagerUnfocusSceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnfocusSceneError::NotRegistered => write!(f, "The scene is not registered."),
            UIManagerUnfocusSceneError::UnfocusFocusedCanvasError(e) => write!(f, "An error occurred while unfocusing the currently focused canvas: {}", e),
        }
    }
}

impl Error for UIManagerUnfocusSceneError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusCanvasError {
    NotRegistered,
    UnfocusFocusedWindowError(UIManagerUnfocusWindowError),
}

impl fmt::Display for UIManagerUnfocusCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnfocusCanvasError::NotRegistered => write!(f, "The canvas is not registered."),
            UIManagerUnfocusCanvasError::UnfocusFocusedWindowError(e) => write!(f, "An error occurred while unfocusing the currently focused window: {}", e),
        }
    }
}

impl Error for UIManagerUnfocusCanvasError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusWindowError {
    NotRegistered,
    UnfocusFocusedContainerError(UIManagerUnfocusContainerError),
}

impl fmt::Display for UIManagerUnfocusWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnfocusWindowError::NotRegistered => write!(f, "The window is not registered."),
            UIManagerUnfocusWindowError::UnfocusFocusedContainerError(e) => write!(f, "An error occurred while unfocusing the currently focused container: {}", e),
        }
    }
}

impl Error for UIManagerUnfocusWindowError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusContainerError {
    NotRegistered,
    UnfocusFocusedElementError(UIManagerUnfocusElementError),
}

impl fmt::Display for UIManagerUnfocusContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnfocusContainerError::NotRegistered => write!(f, "The container is not registered."),
            UIManagerUnfocusContainerError::UnfocusFocusedElementError(e) => write!(f, "An error occurred while unfocusing the currently focused element: {}", e),
        }
    }
}

impl Error for UIManagerUnfocusContainerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusElementError {
    NotRegistered,
}

impl fmt::Display for UIManagerUnfocusElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerUnfocusElementError::NotRegistered => write!(f, "The element is not registered."),
        }
    }
}

impl Error for UIManagerUnfocusElementError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedSceneIDError {
    AllIDsInUse,
}

impl fmt::Display for UIManagerGetUnusedSceneIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerGetUnusedSceneIDError::AllIDsInUse => write!(f, "All scene IDs are in use."),
        }
    }
}

impl Error for UIManagerGetUnusedSceneIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleSceneIDError {
    AlreadyRecycled,
    StillInUse,
}

impl fmt::Display for UIManagerRecycleSceneIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRecycleSceneIDError::AlreadyRecycled => write!(f, "The scene ID is already recycled."),
            UIManagerRecycleSceneIDError::StillInUse => write!(f, "The scene ID is still in use."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedCanvasIDError {
    AllIDsInUse,
}

impl fmt::Display for UIManagerGetUnusedCanvasIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerGetUnusedCanvasIDError::AllIDsInUse => write!(f, "All canvas IDs are in use."),
        }
    }
}

impl Error for UIManagerGetUnusedCanvasIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleCanvasIDError {
    AlreadyRecycled,
    StillInUse,
}

impl fmt::Display for UIManagerRecycleCanvasIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRecycleCanvasIDError::AlreadyRecycled => write!(f, "The canvas ID is already recycled."),
            UIManagerRecycleCanvasIDError::StillInUse => write!(f, "The canvas ID is still in use."),
        }
    }
}

impl Error for UIManagerRecycleCanvasIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedWindowIDError {
    AllIDsInUse,
}

impl fmt::Display for UIManagerGetUnusedWindowIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerGetUnusedWindowIDError::AllIDsInUse => write!(f, "All window IDs are in use."),
        }
    }
}

impl Error for UIManagerGetUnusedWindowIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleWindowIDError {
    AlreadyRecycled,
    StillInUse,
}

impl fmt::Display for UIManagerRecycleWindowIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRecycleWindowIDError::AlreadyRecycled => write!(f, "The window ID is already recycled."),
            UIManagerRecycleWindowIDError::StillInUse => write!(f, "The window ID is still in use."),
        }
    }
}

impl Error for UIManagerRecycleWindowIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedContainerIDError {
    AllIDsInUse,
}

impl fmt::Display for UIManagerGetUnusedContainerIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerGetUnusedContainerIDError::AllIDsInUse => write!(f, "All container IDs are in use."),
        }
    }
}

impl Error for UIManagerGetUnusedContainerIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleContainerIDError {
    AlreadyRecycled,
    StillInUse,
}

impl fmt::Display for UIManagerRecycleContainerIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRecycleContainerIDError::AlreadyRecycled => write!(f, "The container ID is already recycled."),
            UIManagerRecycleContainerIDError::StillInUse => write!(f, "The container ID is still in use."),
        }
    }
}

impl Error for UIManagerRecycleContainerIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedElementIDError {
    AllIDsInUse,
}

impl fmt::Display for UIManagerGetUnusedElementIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerGetUnusedElementIDError::AllIDsInUse => write!(f, "All element IDs are in use."),
        }
    }
}

impl Error for UIManagerGetUnusedElementIDError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleElementIDError {
    AlreadyRecycled,
    StillInUse,
}

impl fmt::Display for UIManagerRecycleElementIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIManagerRecycleElementIDError::AlreadyRecycled => write!(f, "The element ID is already recycled."),
            UIManagerRecycleElementIDError::StillInUse => write!(f, "The element ID is still in use."),
        }
    }
}

impl Error for UIManagerRecycleElementIDError {}

#[derive(Default)]
pub struct UIManager {
    registered_scene_types: HashSet<TypeId>,
    registered_canvas_types: HashSet<TypeId>,
    registered_window_types: HashSet<TypeId>,
    registered_container_types: HashSet<TypeId>,
    registered_element_types: HashSet<TypeId>,

    new_scene_id: UISceneID,
    new_canvas_id: UICanvasID,
    new_window_id: UIWindowID,
    new_container_id: UIContainerID,
    new_element_id: UIElementID,

    recycled_scene_ids: Vec<UISceneID>,
    recycled_canvas_ids: Vec<UICanvasID>,
    recycled_window_ids: Vec<UIWindowID>,
    recycled_container_ids: Vec<UIContainerID>,
    recycled_element_ids: Vec<UIElementID>,
    
    registered_scenes: HashMap<UISceneID, Arc<Mutex<dyn 'static + UIScene>>>,
    registered_canvases: HashMap<UICanvasID, Arc<Mutex<dyn 'static + UICanvas>>>,
    registered_windows: HashMap<UIWindowID, Arc<Mutex<dyn 'static + UIWindow>>>,
    registered_containers: HashMap<UIContainerID, Arc<Mutex<dyn 'static + UIContainer>>>,
    registered_elements: HashMap<UIElementID, Arc<Mutex<dyn 'static + UIElement>>>,

    focused_scene_id: Option<UISceneID>,
    focused_canvas_id: Option<UICanvasID>,
    focused_window_id: Option<UIWindowID>,
    focused_container_id: Option<UIContainerID>,
    focused_element_id: Option<UIElementID>,
}

// TODO: Finish implementing the UIManager (review all methods and error types, and ensure that the system is fully functional and error-proof)
impl UIManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_scene_type<T: 'static + UIScene>(&mut self) -> Result<(), UIManagerRegisterSceneTypeError> {
        if self.registered_scene_types.insert(TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerRegisterSceneTypeError::AlreadyRegistered)
        }
    }

    pub fn register_canvas_type<T: 'static + UICanvas>(&mut self) -> Result<(), UIManagerRegisterCanvasTypeError> {
        if self.registered_canvas_types.insert(TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerRegisterCanvasTypeError::AlreadyRegistered)
        }
    }

    pub fn register_window_type<T: 'static + UIWindow>(&mut self) -> Result<(), UIManagerRegisterWindowTypeError> {
        if self.registered_window_types.insert(TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerRegisterWindowTypeError::AlreadyRegistered)
        }
    }

    pub fn register_container_type<T: 'static + UIContainer>(&mut self) -> Result<(), UIManagerRegisterContainerTypeError> {
        if self.registered_container_types.insert(TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerRegisterContainerTypeError::AlreadyRegistered)
        }
    }

    pub fn register_element_type<T: 'static + UIElement>(&mut self) -> Result<(), UIManagerRegisterElementTypeError> {
        if self.registered_element_types.insert(TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerRegisterElementTypeError::AlreadyRegistered)
        }
    }

    pub fn unregister_scene_type<T: 'static + UIScene>(&mut self) -> Result<(), UIManagerUnregisterSceneTypeError> {
        if self.registered_scene_types.remove(&TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerUnregisterSceneTypeError::AlreadyUnregistered)
        }
    }

    pub fn unregister_canvas_type<T: 'static + UICanvas>(&mut self) -> Result<(), UIManagerUnregisterCanvasTypeError> {
        if self.registered_canvas_types.remove(&TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerUnregisterCanvasTypeError::AlreadyUnregistered)
        }
    }

    pub fn unregister_window_type<T: 'static + UIWindow>(&mut self) -> Result<(), UIManagerUnregisterWindowTypeError> {
        if self.registered_window_types.remove(&TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerUnregisterWindowTypeError::AlreadyUnregistered)
        }
    }

    pub fn unregister_container_type<T: 'static + UIContainer>(&mut self) -> Result<(), UIManagerUnregisterContainerTypeError> {
        if self.registered_container_types.remove(&TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerUnregisterContainerTypeError::AlreadyUnregistered)
        }
    }

    pub fn unregister_element_type<T: 'static + UIElement>(&mut self) -> Result<(), UIManagerUnregisterElementTypeError> {
        if self.registered_element_types.remove(&TypeId::of::<T>()) {
            Ok(())
        } else {
            Err(UIManagerUnregisterElementTypeError::AlreadyUnregistered)
        }
    }

    pub fn is_scene_type_registered<T: 'static + UIScene>(&self) -> bool {
        self.registered_scene_types.contains(&TypeId::of::<T>())
    }

    pub fn is_canvas_type_registered<T: 'static + UICanvas>(&self) -> bool {
        self.registered_canvas_types.contains(&TypeId::of::<T>())
    }

    pub fn is_window_type_registered<T: 'static + UIWindow>(&self) -> bool {
        self.registered_window_types.contains(&TypeId::of::<T>())
    }

    pub fn is_container_type_registered<T: 'static + UIContainer>(&self) -> bool {
        self.registered_container_types.contains(&TypeId::of::<T>())
    }

    pub fn is_element_type_registered<T: 'static + UIElement>(&self) -> bool {
        self.registered_element_types.contains(&TypeId::of::<T>())
    }

    fn get_unused_scene_id(&mut self) -> Result<UISceneID, UIManagerGetUnusedSceneIDError> {
        let unused_scene_id = match self.recycled_scene_ids.pop() {
            Some(recycled_scene_id) => recycled_scene_id,
            None => {
                if self.new_scene_id == usize::MAX {
                    return Err(UIManagerGetUnusedSceneIDError::AllIDsInUse);
                }

                let new_scene_id = self.new_scene_id;

                self.new_scene_id += 1;

                new_scene_id
            },
        };

        Ok(unused_scene_id)
    }

    fn recycle_scene_id(&mut self, scene_id: UISceneID) -> Result<(), UIManagerRecycleSceneIDError> {
        if self.recycled_scene_ids.contains(&scene_id) {
            return Err(UIManagerRecycleSceneIDError::AlreadyRecycled);
        }

        if self.registered_scenes.contains_key(&scene_id) {
            return Err(UIManagerRecycleSceneIDError::StillInUse);
        }

        self.recycled_scene_ids.push(scene_id);

        Ok(())
    }

    fn get_unused_canvas_id(&mut self) -> Result<UICanvasID, UIManagerGetUnusedCanvasIDError> {
        let unused_canvas_id = match self.recycled_canvas_ids.pop() {
            Some(recycled_canvas_id) => recycled_canvas_id,
            None => {
                if self.new_canvas_id == usize::MAX {
                    return Err(UIManagerGetUnusedCanvasIDError::AllIDsInUse);
                }

                let new_canvas_id = self.new_canvas_id;

                self.new_canvas_id += 1;

                new_canvas_id
            },
        };

        Ok(unused_canvas_id)
    }

    fn recycle_canvas_id(&mut self, canvas_id: UICanvasID) -> Result<(), UIManagerRecycleCanvasIDError> {
        if self.recycled_canvas_ids.contains(&canvas_id) {
            return Err(UIManagerRecycleCanvasIDError::AlreadyRecycled);
        }

        if self.registered_canvases.contains_key(&canvas_id) {
            return Err(UIManagerRecycleCanvasIDError::StillInUse);
        }

        self.recycled_canvas_ids.push(canvas_id);

        Ok(())
    }

    fn get_unused_window_id(&mut self) -> Result<UIWindowID, UIManagerGetUnusedWindowIDError> {
        let unused_window_id = match self.recycled_window_ids.pop() {
            Some(recycled_window_id) => recycled_window_id,
            None => {
                if self.new_window_id == usize::MAX {
                    return Err(UIManagerGetUnusedWindowIDError::AllIDsInUse);
                }

                let new_window_id = self.new_window_id;

                self.new_window_id += 1;

                new_window_id
            },
        };

        Ok(unused_window_id)
    }

    fn recycle_window_id(&mut self, window_id: UIWindowID) -> Result<(), UIManagerRecycleWindowIDError> {
        if self.recycled_window_ids.contains(&window_id) {
            return Err(UIManagerRecycleWindowIDError::AlreadyRecycled);
        }

        if self.registered_windows.contains_key(&window_id) {
            return Err(UIManagerRecycleWindowIDError::StillInUse);
        }

        self.recycled_window_ids.push(window_id);

        Ok(())
    }

    fn get_unused_container_id(&mut self) -> Result<UIContainerID, UIManagerGetUnusedContainerIDError> {
        let unused_container_id = match self.recycled_container_ids.pop() {
            Some(recycled_container_id) => recycled_container_id,
            None => {
                if self.new_container_id == usize::MAX {
                    return Err(UIManagerGetUnusedContainerIDError::AllIDsInUse);
                }

                let new_container_id = self.new_container_id;

                self.new_container_id += 1;

                new_container_id
            },
        };

        Ok(unused_container_id)
    }

    fn recycle_container_id(&mut self, container_id: UIContainerID) -> Result<(), UIManagerRecycleContainerIDError> {
        if self.recycled_container_ids.contains(&container_id) {
            return Err(UIManagerRecycleContainerIDError::AlreadyRecycled);
        }

        if self.registered_containers.contains_key(&container_id) {
            return Err(UIManagerRecycleContainerIDError::StillInUse);
        }

        self.recycled_container_ids.push(container_id);

        Ok(())
    }

    fn get_unused_element_id(&mut self) -> Result<UIElementID, UIManagerGetUnusedElementIDError> {
        let unused_element_id = match self.recycled_element_ids.pop() {
            Some(recycled_element_id) => recycled_element_id,
            None => {
                if self.new_element_id == usize::MAX {
                    return Err(UIManagerGetUnusedElementIDError::AllIDsInUse);
                }

                let new_element_id = self.new_element_id;

                self.new_element_id += 1;

                new_element_id
            },
        };

        Ok(unused_element_id)
    }

    fn recycle_element_id(&mut self, element_id: UIElementID) -> Result<(), UIManagerRecycleElementIDError> {
        if self.recycled_element_ids.contains(&element_id) {
            return Err(UIManagerRecycleElementIDError::AlreadyRecycled);
        }

        if self.registered_elements.contains_key(&element_id) {
            return Err(UIManagerRecycleElementIDError::StillInUse);
        }

        self.recycled_element_ids.push(element_id);

        Ok(())
    }

    pub fn register_scene<T: 'static + UIScene>(&mut self, mut scene: T) -> Result<UISceneID, UIManagerRegisterSceneError> {
        if !self.is_scene_type_registered::<T>() {
            return Err(UIManagerRegisterSceneError::TypeNotRegistered);
        }
        
        if scene.get_id().is_some() {
            return Err(UIManagerRegisterSceneError::AlreadyRegistered);
        }

        let scene_id = match self.get_unused_scene_id() {
            Ok(unused_scene_id) => {
                scene.set_id(Some(unused_scene_id));

                unused_scene_id
            },
            Err(e) => return Err(UIManagerRegisterSceneError::GetUnusedSceneIDError(e)),
        };

        if self.registered_scenes.contains_key(&scene_id) {
            panic!("Unused scene ID is actually already registered!");
        }

        scene.set_id(Some(scene_id));

        self.registered_scenes.insert(scene_id, Arc::new(Mutex::new(scene)));

        Ok(scene_id)
    }

    pub fn register_canvas<T: 'static + UICanvas>(&mut self, mut canvas: T) -> Result<UICanvasID, UIManagerRegisterCanvasError> {
        if !self.is_canvas_type_registered::<T>() {
            return Err(UIManagerRegisterCanvasError::TypeNotRegistered);
        }

        if canvas.get_id().is_some() {
            return Err(UIManagerRegisterCanvasError::AlreadyRegistered);
        }

        let parent_scene_id = match canvas.get_parent() {
            Some(parent_scene_id) => parent_scene_id,
            None => return Err(UIManagerRegisterCanvasError::NoParentScene),
        };

        if !self.registered_scenes.contains_key(&parent_scene_id) {
            return Err(UIManagerRegisterCanvasError::ParentSceneNotRegistered);
        }

        let canvas_id = match self.get_unused_canvas_id() {
            Ok(unused_canvas_id) => {
                canvas.set_id(Some(unused_canvas_id));

                unused_canvas_id
            },
            Err(e) => return Err(UIManagerRegisterCanvasError::GetUnusedCanvasIDError(e)),
        };

        if self.registered_canvases.contains_key(&canvas_id) {
            panic!("Unused canvas ID is actually already registered!");
        }

        canvas.set_id(Some(canvas_id));

        self.registered_canvases.insert(canvas_id, Arc::new(Mutex::new(canvas)));

        Ok(canvas_id)
    }

    pub fn register_window<T: 'static + UIWindow>(&mut self, mut window: T) -> Result<UIWindowID, UIManagerRegisterWindowError> {
        if !self.is_window_type_registered::<T>() {
            return Err(UIManagerRegisterWindowError::TypeNotRegistered);
        }

        if window.get_id().is_some() {
            return Err(UIManagerRegisterWindowError::AlreadyRegistered);
        }

        let parent_canvas_id = match window.get_parent() {
            Some(parent_canvas_id) => parent_canvas_id,
            None => return Err(UIManagerRegisterWindowError::NoParentCanvas),
        };

        if !self.registered_canvases.contains_key(&parent_canvas_id) {
            return Err(UIManagerRegisterWindowError::ParentCanvasNotRegistered);
        }

        let window_id = match self.get_unused_window_id() {
            Ok(unused_window_id) => {
                window.set_id(Some(unused_window_id));

                unused_window_id
            },
            Err(e) => return Err(UIManagerRegisterWindowError::GetUnusedWindowIDError(e)),
        };

        if self.registered_windows.contains_key(&window_id) {
            panic!("Unused window ID is actually already registered!");
        }

        window.set_id(Some(window_id));

        self.registered_windows.insert(window_id, Arc::new(Mutex::new(window)));

        Ok(window_id)
    }

    pub fn register_container<T: 'static + UIContainer>(&mut self, mut container: T) -> Result<UIContainerID, UIManagerRegisterContainerError> {
        if !self.is_container_type_registered::<T>() {
            return Err(UIManagerRegisterContainerError::TypeNotRegistered);
        }

        if container.get_id().is_some() {
            return Err(UIManagerRegisterContainerError::AlreadyRegistered);
        }

        let (parent_type, parent_id) = match container.get_parent() {
            Some((parent_type, parent_id)) => (parent_type, parent_id),
            None => return Err(UIManagerRegisterContainerError::NoParent),
        };

        match parent_type {
            UIContainerParentType::Window => {
                if !self.registered_windows.contains_key(&parent_id) {
                    return Err(UIManagerRegisterContainerError::ParentWindowNotRegistered);
                }
            },
            UIContainerParentType::Container => {
                if !self.registered_containers.contains_key(&parent_id) {
                    return Err(UIManagerRegisterContainerError::ParentContainerNotRegistered);
                }
            },
        };

        let container_id = match self.get_unused_container_id() {
            Ok(unused_container_id) => {
                container.set_id(Some(unused_container_id));

                unused_container_id
            },
            Err(e) => return Err(UIManagerRegisterContainerError::GetUnusedContainerIDError(e)),
        };

        if self.registered_containers.contains_key(&container_id) {
            panic!("Unused container ID is actually already registered!");
        }

        container.set_id(Some(container_id));

        self.registered_containers.insert(container_id, Arc::new(Mutex::new(container)));

        Ok(container_id)
    }

    pub fn register_element<T: 'static + UIElement>(&mut self, mut element: T) -> Result<UIElementID, UIManagerRegisterElementError> {
        if !self.is_element_type_registered::<T>() {
            return Err(UIManagerRegisterElementError::TypeNotRegistered);
        }

        if element.get_id().is_some() {
            return Err(UIManagerRegisterElementError::AlreadyRegistered);
        }

        let parent_container_id = match element.get_parent() {
            Some(parent_container_id) => parent_container_id,
            None => return Err(UIManagerRegisterElementError::NoParentContainer),
        };

        if !self.registered_containers.contains_key(&parent_container_id) {
            return Err(UIManagerRegisterElementError::ParentContainerNotRegistered);
        }

        let element_id = match self.get_unused_element_id() {
            Ok(unused_element_id) => {
                element.set_id(Some(unused_element_id));

                unused_element_id
            },
            Err(e) => return Err(UIManagerRegisterElementError::GetUnusedElementIDError(e)),
        };

        if self.registered_elements.contains_key(&element_id) {
            panic!("Unused element ID is actually already registered!");
        }

        element.set_id(Some(element_id));

        self.registered_elements.insert(element_id, Arc::new(Mutex::new(element)));

        Ok(element_id)
    }

    pub fn unregister_scene(&mut self, scene_id: UISceneID) -> Result<(), UIManagerUnregisterSceneError> {
        let removed_scene = match self.registered_scenes.remove(&scene_id) {
            Some(removed_scene) => removed_scene,
            None => {
                return Err(UIManagerUnregisterSceneError::AlreadyUnregistered);
            }
        };

        let mut removed_scene = match removed_scene.lock() {
            Ok(removed_scene) => removed_scene,
            Err(_) => panic!("Scene mutex is poisoned!"),
        };

        // TODO: Unregister child canvases

        removed_scene.set_id(None);

        drop(removed_scene);

        match self.recycle_scene_id(scene_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(UIManagerUnregisterSceneError::RecycleSceneIDError(e)),
        }
    }

    pub fn unregister_canvas(&mut self, canvas_id: UICanvasID) -> Result<(), UIManagerUnregisterCanvasError> {
        let removed_canvas = match self.registered_canvases.remove(&canvas_id) {
            Some(removed_canvas) => removed_canvas,
            None => {
                return Err(UIManagerUnregisterCanvasError::AlreadyUnregistered);
            }
        };

        let mut removed_canvas = match removed_canvas.lock() {
            Ok(removed_canvas) => removed_canvas,
            Err(_) => panic!("Canvas mutex is poisoned!"),
        };

        // TODO: Unregister child windows

        removed_canvas.set_id(None);

        drop(removed_canvas);

        match self.recycle_canvas_id(canvas_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(UIManagerUnregisterCanvasError::RecycleCanvasIDError(e)),
        }
    }

    pub fn unregister_window(&mut self, window_id: UIWindowID) -> Result<(), UIManagerUnregisterWindowError> {
        let removed_window = match self.registered_windows.remove(&window_id) {
            Some(removed_window) => removed_window,
            None => {
                return Err(UIManagerUnregisterWindowError::AlreadyUnregistered);
            }
        };

        let mut removed_window = match removed_window.lock() {
            Ok(removed_window) => removed_window,
            Err(_) => panic!("Window mutex is poisoned!"),
        };

        // TODO: Unregister child containers

        removed_window.set_id(None);

        drop(removed_window);

        match self.recycle_window_id(window_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(UIManagerUnregisterWindowError::RecycleWindowIDError(e)),
        }
    }

    pub fn unregister_container(&mut self, container_id: UIContainerID) -> Result<(), UIManagerUnregisterContainerError> {
        let removed_container = match self.registered_containers.remove(&container_id) {
            Some(container) => container,
            None => {
                return Err(UIManagerUnregisterContainerError::AlreadyUnregistered);
            }
        };

        let mut removed_container = match removed_container.lock() {
            Ok(removed_container) => removed_container,
            Err(_) => panic!("Container mutex is poisoned!"),
        };

        // TODO: Unregister child elements

        removed_container.set_id(None);

        drop(removed_container);

        match self.recycle_container_id(container_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(UIManagerUnregisterContainerError::RecycleContainerIDError(e)),
        }
    }

    pub fn unregister_element(&mut self, element_id: UIElementID) -> Result<(), UIManagerUnregisterElementError> {
        let removed_element = match self.registered_elements.remove(&element_id) {
            Some(element) => element,
            None => {
                return Err(UIManagerUnregisterElementError::AlreadyUnregistered);
            }
        };

        let mut removed_element = match removed_element.lock() {
            Ok(removed_element) => removed_element,
            Err(_) => panic!("Element mutex is poisoned!"),
        };

        removed_element.set_id(None);

        drop(removed_element);

        match self.recycle_element_id(element_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(UIManagerUnregisterElementError::RecycleElementIDError(e)),
        }
    }

    pub fn is_scene_registered(&self, scene_id: &UISceneID) -> bool {
        self.registered_scenes.contains_key(scene_id)
    }

    pub fn is_canvas_registered(&self, canvas_id: &UICanvasID) -> bool {
        self.registered_canvases.contains_key(canvas_id)
    }

    pub fn is_window_registered(&self, window_id: &UIWindowID) -> bool {
        self.registered_windows.contains_key(window_id)
    }

    pub fn is_container_registered(&self, container_id: &UIContainerID) -> bool {
        self.registered_containers.contains_key(container_id)
    }

    pub fn is_element_registered(&self, element_id: &UIElementID) -> bool {
        self.registered_elements.contains_key(element_id)
    }

    pub fn get_scene(&self, scene_id: &UISceneID) -> Option<Arc<Mutex<dyn 'static + UIScene>>> {
        self.registered_scenes.get(scene_id).cloned()
    }

    pub fn get_canvas(&self, canvas_id: &UICanvasID) -> Option<Arc<Mutex<dyn 'static + UICanvas>>> {
        self.registered_canvases.get(canvas_id).cloned()
    }

    pub fn get_window(&self, window_id: &UIWindowID) -> Option<Arc<Mutex<dyn 'static + UIWindow>>> {
        self.registered_windows.get(window_id).cloned()
    }

    pub fn get_container(&self, container_id: &UIContainerID) -> Option<Arc<Mutex<dyn 'static + UIContainer>>> {
        self.registered_containers.get(container_id).cloned()
    }

    pub fn get_element(&self, element_id: &UIElementID) -> Option<Arc<Mutex<dyn 'static + UIElement>>> {
        self.registered_elements.get(element_id).cloned()
    }

    pub fn focus_scene(&mut self, scene_id: &UISceneID) -> Result<(), UIManagerFocusSceneError> {
        if let Some(focused_scene_id) = self.focused_scene_id {
            if focused_scene_id == *scene_id {
                return Ok(());
            }
        };

        let scene = match self.get_scene(scene_id) {
            Some(scene) => scene,
            None => return Err(UIManagerFocusSceneError::NotRegistered),
        };

        let scene = match scene.lock() {
            Ok(scene) => scene,
            Err(_) => panic!("Scene mutex is poisoned!"),
        };

        match self.unfocus_scene() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerFocusSceneError::UnfocusFocusedSceneError(e)),
        };

        self.focused_scene_id = Some(*scene_id);

        scene.on_focus();

        Ok(())
    }

    pub fn focus_canvas(&mut self, canvas_id: &UICanvasID) -> Result<(), UIManagerFocusCanvasError> {
        if let Some(focused_canvas_id) = self.focused_canvas_id {
            if focused_canvas_id == *canvas_id {
                return Ok(());
            }
        };

        let canvas = match self.get_canvas(canvas_id) {
            Some(canvas) => canvas.clone(),
            None => return Err(UIManagerFocusCanvasError::NotRegistered),
        };

        let canvas = match canvas.lock() {
            Ok(canvas) => canvas,
            Err(_) => panic!("Canvas mutex is poisoned!"),
        };

        match self.unfocus_canvas() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerFocusCanvasError::UnfocusFocusedCanvasError(e)),
        };
        
        self.focused_canvas_id = Some(*canvas_id);

        canvas.on_focus();

        Ok(())
    }

    pub fn focus_window(&mut self, window_id: &UIWindowID) -> Result<(), UIManagerFocusWindowError> {
        if let Some(focused_window_id) = self.focused_window_id {
            if focused_window_id == *window_id {
                return Ok(());
            }
        };

        let window = match self.get_window(window_id) {
            Some(window) => window.clone(),
            None => return Err(UIManagerFocusWindowError::NotRegistered),
        };

        let window = match window.lock() {
            Ok(window) => window,
            Err(_) => panic!("Window mutex is poisoned!"),
        };

        match self.unfocus_window() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerFocusWindowError::UnfocusFocusedWindowError(e)),
        };

        self.focused_window_id = Some(*window_id);

        window.on_focus();

        Ok(())
    }

    pub fn focus_container(&mut self, container_id: &UIContainerID) -> Result<(), UIManagerFocusContainerError> {
        if let Some(focused_container_id) = self.focused_container_id {
            if focused_container_id == *container_id {
                return Ok(());
            }
        };

        let container = match self.get_container(container_id) {
            Some(container) => container.clone(),
            None => return Err(UIManagerFocusContainerError::NotRegistered),
        };

        let container = match container.lock() {
            Ok(container) => container,
            Err(_) => panic!("Container mutex is poisoned!"),
        };

        match self.unfocus_container() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerFocusContainerError::UnfocusFocusedContainerError(e)),
        };

        self.focused_container_id = Some(*container_id);

        container.on_focus();

        Ok(())
    }

    pub fn focus_element(&mut self, element_id: &UIElementID) -> Result<(), UIManagerFocusElementError> {
        if let Some(focused_element_id) = self.focused_element_id {
            if focused_element_id == *element_id {
                return Ok(());
            }
        };

        let element = match self.get_element(element_id) {
            Some(element) => element.clone(),
            None => return Err(UIManagerFocusElementError::NotRegistered),
        };

        let element = match element.lock() {
            Ok(element) => element,
            Err(_) => panic!("Element mutex is poisoned!"),
        };

        match self.unfocus_element() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerFocusElementError::UnfocusFocusedElementError(e)),
        };

        self.focused_element_id = Some(*element_id);

        element.on_focus();

        Ok(())
    }

    pub fn unfocus_scene(&mut self) -> Result<(), UIManagerUnfocusSceneError> {
        let scene_id = match self.focused_scene_id {
            Some(scene_id) => scene_id,
            None => return Ok(()),
        };

        let scene = match self.get_scene(&scene_id) {
            Some(scene) => scene.clone(),
            None => return Err(UIManagerUnfocusSceneError::NotRegistered),
        };

        let scene = match scene.lock() {
            Ok(scene) => scene,
            Err(_) => panic!("Scene mutex is poisoned!"),
        };

        match self.unfocus_canvas() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerUnfocusSceneError::UnfocusFocusedCanvasError(e)),
        };

        self.focused_scene_id = None;

        scene.on_unfocus();

        Ok(())
    }

    pub fn unfocus_canvas(&mut self) -> Result<(), UIManagerUnfocusCanvasError> {
        let canvas_id = match self.focused_canvas_id {
            Some(canvas_id) => canvas_id,
            None => return Ok(()),
        };

        let canvas = match self.get_canvas(&canvas_id) {
            Some(canvas) => canvas.clone(),
            None => return Err(UIManagerUnfocusCanvasError::NotRegistered),
        };

        let canvas = match canvas.lock() {
            Ok(canvas) => canvas,
            Err(_) => panic!("Canvas mutex is poisoned!"),
        };

        match self.unfocus_window() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerUnfocusCanvasError::UnfocusFocusedWindowError(e)),
        };

        self.focused_canvas_id = None;

        canvas.on_unfocus();

        Ok(())
    }

    pub fn unfocus_window(&mut self) -> Result<(), UIManagerUnfocusWindowError> {
        let window_id = match self.focused_window_id {
            Some(window_id) => window_id,
            None => return Ok(()),
        };

        let window = match self.get_window(&window_id) {
            Some(window) => window.clone(),
            None => return Err(UIManagerUnfocusWindowError::NotRegistered),
        };

        let window = match window.lock() {
            Ok(window) => window,
            Err(_) => panic!("Window mutex is poisoned!"),
        };

        match self.unfocus_container() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerUnfocusWindowError::UnfocusFocusedContainerError(e)),
        };

        self.focused_window_id = None;

        window.on_unfocus();

        Ok(())
    }

    pub fn unfocus_container(&mut self) -> Result<(), UIManagerUnfocusContainerError> {
        let container_id = match self.focused_container_id {
            Some(container_id) => container_id,
            None => return Ok(()),
        };

        let container = match self.get_container(&container_id) {
            Some(container) => container.clone(),
            None => return Err(UIManagerUnfocusContainerError::NotRegistered),
        };

        let container = match container.lock() {
            Ok(container) => container,
            Err(_) => panic!("Container mutex is poisoned!"),
        };

        match self.unfocus_element() {
            Ok(_) => {},
            Err(e) => return Err(UIManagerUnfocusContainerError::UnfocusFocusedElementError(e)),
        };

        self.focused_container_id = None;
        
        container.on_unfocus();

        Ok(())
    }

    pub fn unfocus_element(&mut self) -> Result<(), UIManagerUnfocusElementError> {
        let element_id = match self.focused_element_id {
            Some(element_id) => element_id,
            None => return Ok(()),
        };

        let element = match self.get_element(&element_id) {
            Some(element) => element.clone(),
            None => return Err(UIManagerUnfocusElementError::NotRegistered),
        };

        let element = match element.lock() {
            Ok(element) => element,
            Err(_) => panic!("Element mutex is poisoned!"),
        };

        self.focused_element_id = None;

        element.on_unfocus();

        Ok(())
    }

    pub fn is_scene_focused(&self, scene_id: &UISceneID) -> bool {
        self.focused_scene_id == Some(*scene_id)
    }

    pub fn is_canvas_focused(&self, canvas_id: &UICanvasID) -> bool {
        self.focused_canvas_id == Some(*canvas_id)
    }

    pub fn is_window_focused(&self, window_id: &UIWindowID) -> bool {
        self.focused_window_id == Some(*window_id)
    }

    pub fn is_container_focused(&self, container_id: &UIContainerID) -> bool {
        self.focused_container_id == Some(*container_id)
    }

    pub fn is_element_focused(&self, element_id: &UIElementID) -> bool {
        self.focused_element_id == Some(*element_id)
    }

    pub fn get_focused_scene_id(&self) -> Option<Arc<Mutex<(dyn 'static + UIScene)>>> {
        self.focused_scene_id.and_then(|scene_id| self.registered_scenes.get(&scene_id)).cloned()
    }

    pub fn get_focused_canvas_id(&self) -> Option<Arc<Mutex<(dyn 'static + UICanvas)>>> {
        self.focused_canvas_id.and_then(|canvas_id| self.registered_canvases.get(&canvas_id)).cloned()
    }

    pub fn get_focused_window_id(&self) -> Option<Arc<Mutex<(dyn 'static + UIWindow)>>> {
        self.focused_window_id.and_then(|window_id| self.registered_windows.get(&window_id)).cloned()
    }

    pub fn get_focused_container_id(&self) -> Option<Arc<Mutex<(dyn 'static + UIContainer)>>> {
        self.focused_container_id.and_then(|container_id| self.registered_containers.get(&container_id)).cloned()
    }

    pub fn get_focused_element_id(&self) -> Option<Arc<Mutex<(dyn 'static + UIElement)>>> {
        self.focused_element_id.and_then(|element_id| self.registered_elements.get(&element_id)).cloned()
    }
}

/*
TODO: Implement Advanced UI System (RAW USER CREATED)

1. 6-Layer Event Management with Prioritization
    - Event Propagation: Events are passed down from the top of the UI hierarchy (Scene) to the targeted UI Object, with the ability for any EventHandler to consume the event, thus preventing it from propagating further. This allows for precise control over how events are handled and ensures that only the most relevant object processes the event.
    - Event Handling Layers: The system distinguishes between key press events, which are sent to all focused objects across the four focusable layers, and other types of events, which target specific UI Objects based on their position in the hierarchy and the event's specified target path.
    - Event Targeting: Non-key press events require specifying the intended target UI Object's "String" (a path through the UI hierarchy), enabling the UIManager to route the event through the hierarchy (Scene > Canvas > Window > Container > Element > Component) to the specified target. This mechanism ensures that events are directed accurately to their intended recipients.
    - Event Consumption: Elements have the ability to consume events, preventing further propagation to their components. However, components cannot consume events, as they are not directly focusable. This maintains a clear separation between Elements (which can interact with and potentially halt events) and Components (which can react to events but not stop their propagation).

2. Implementation Details
    "Event Handling Layers: The system distinguishes between key press events, which are sent to all focused objects across the four focusable layers, and other types of events, which target specific UI Objects based on their position in the hierarchy and the event's specified target path."
    To be precise: The key press events are an entirely different system from the ui events, because they are sent in a completely different, unconsumable way.
*/

/*
TODO: Implement Advanced UI System (AI "enhanced")

1. 6-Layer Event Management System with Prioritization:
    - Develop an event management system that allows events to be routed from the top of the UI hierarchy downwards towards the intended target object, specified by a unique path through the UI hierarchy.
    - Distinguish between key press events and other UI events:
        - Key Press Events System: Implement a distinct system for key press events where such events are sent to all focused objects across the four focusable layers (Canvas, Window, Container, Element) in a non-consumable manner. This ensures that key press events are universally accessible and not subject to consumption by any single UI object.
        - UI Events System: For all other types of events, implement a targeted delivery system where events are directed to a specific UI Object (or set of objects) based on their hierarchy and a specified path. This system allows events to be consumed by the target object, preventing further propagation.
    - Include functionality for EventHandler objects to consume an event, halting further propagation to children, with the mechanism for specifying the target UI Object's path within the event itself to facilitate precise routing.
    - Ensure that events can be routed accurately through the hierarchy from Scene to Canvas, Window, Container, Element, and finally to Components, unless consumed by the Element. Note that Components cannot consume events, reflecting their role as part of a larger interactive Element.

2. Implementation Details:
    - Create robust interfaces and base classes for UI objects that include methods for startup, shutdown, focusing, and event handling, adhering to the principles of the focus and event management systems described above.
    - Design the UIManager to effectively manage the current scene, focus states across different layers, and the routing and handling of events within the UI hierarchy.
    - Pay special attention to the separation of key press events from other UI events in the system's design to ensure both types of events are handled appropriately and according to their distinct characteristics.

This comprehensive plan aims to create a flexible, efficient, and intuitive UI system that enhances user interaction and event handling within the game's UI architecture.
*/