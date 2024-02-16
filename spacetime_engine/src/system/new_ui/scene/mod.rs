use super::object::*;
use super::canvas::*;
use std::error::Error;
use std::fmt;
use std::any::TypeId;
use std::collections::HashSet;

pub type UISceneID = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UISceneInsertCanvasError {
    AlreadyInserted,
}

impl fmt::Display for UISceneInsertCanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UISceneInsertCanvasError::AlreadyInserted => {
                write!(f, "The canvas is already inserted.")
            }
        }
    }
}

impl Error for UISceneInsertCanvasError {}

#[derive(Clone, Debug, PartialEq, Eq)]
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