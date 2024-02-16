use super::object::*;
use super::container::*;
use super::canvas::*;
use std::error::Error;
use std::fmt;
use std::any::TypeId;
use std::collections::HashSet;

pub type UIWindowID = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIWindowInsertContainerError {
    AlreadyInserted,
}

impl fmt::Display for UIWindowInsertContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIWindowInsertContainerError::AlreadyInserted => {
                write!(f, "The container is already inserted.")
            }
        }
    }
}

impl Error for UIWindowInsertContainerError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIWindowRemoveContainerError {
    AlreadyRemoved,
}

impl fmt::Display for UIWindowRemoveContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIWindowRemoveContainerError::AlreadyRemoved => {
                write!(f, "The container is already removed.")
            }
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

    fn insert_container(
        &mut self,
        container_id: UIContainerID,
    ) -> Result<(), UIWindowInsertContainerError>;
    fn remove_container(
        &mut self,
        container_id: UIContainerID,
    ) -> Result<(), UIWindowRemoveContainerError>;

    fn get_containers(&self) -> &HashSet<UIContainerID>;

    fn contains_container(&self, container_id: UIContainerID) -> bool;
}