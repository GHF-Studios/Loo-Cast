use super::object::*;
use super::window::*;
use super::scene::*;
use std::error::Error;
use std::fmt;
use std::any::TypeId;
use std::collections::HashSet;

pub type UICanvasID = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UICanvasRenderingContext {
    ScreenSpace,
    WorldSpace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UICanvasInsertWindowError {
    AlreadyInserted,
}

impl fmt::Display for UICanvasInsertWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UICanvasInsertWindowError::AlreadyInserted => {
                write!(f, "The window is already inserted.")
            }
        }
    }
}

impl Error for UICanvasInsertWindowError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UICanvasRemoveWindowError {
    AlreadyRemoved,
}

impl fmt::Display for UICanvasRemoveWindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UICanvasRemoveWindowError::AlreadyRemoved => {
                write!(f, "The window is already removed.")
            }
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