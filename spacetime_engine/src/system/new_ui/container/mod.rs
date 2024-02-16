use super::object::*;
use super::element::*;
use super::window::*;
use std::error::Error;
use std::fmt;
use std::any::TypeId;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerParentType {
    Window,
    Container,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerChildType {
    Element,
    Container,
}

pub type UIContainerID = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerInsertElementError {
    AlreadyInserted,
}

impl fmt::Display for UIContainerInsertElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerInsertElementError::AlreadyInserted => {
                write!(f, "The element is already inserted.")
            }
        }
    }
}

impl Error for UIContainerInsertElementError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerInsertContainerError {
    AlreadyInserted,
}

impl fmt::Display for UIContainerInsertContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerInsertContainerError::AlreadyInserted => {
                write!(f, "The container is already inserted.")
            }
        }
    }
}

impl Error for UIContainerInsertContainerError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerRemoveElementError {
    AlreadyRemoved,
}

impl fmt::Display for UIContainerRemoveElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerRemoveElementError::AlreadyRemoved => {
                write!(f, "The element is already removed.")
            }
        }
    }
}

impl Error for UIContainerRemoveElementError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UIContainerRemoveContainerError {
    AlreadyRemoved,
}

impl fmt::Display for UIContainerRemoveContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIContainerRemoveContainerError::AlreadyRemoved => {
                write!(f, "The container is already removed.")
            }
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

    fn insert_element(
        &mut self,
        element_id: UIElementID,
    ) -> Result<(), UIContainerInsertElementError>;
    fn insert_container(
        &mut self,
        container_id: UIContainerID,
    ) -> Result<(), UIContainerInsertContainerError>;

    fn remove_element(
        &mut self,
        element_id: UIElementID,
    ) -> Result<(), UIContainerRemoveElementError>;
    fn remove_container(
        &mut self,
        container_id: UIContainerID,
    ) -> Result<(), UIContainerRemoveContainerError>;

    fn get_elements(&self) -> &HashSet<UIElementID>;
    fn get_containers(&self) -> &HashSet<UIContainerID>;

    fn contains_element(&self, element_id: UIElementID) -> bool;
    fn contains_container(&self, container_id: UIContainerID) -> bool;
}