use super::object::*;
use super::container::*;
use std::any::TypeId;

pub type UIElementID = usize;

pub trait UIElement: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UIElementID>;
    fn set_id(&mut self, element_id: Option<UIElementID>);

    fn get_parent(&self) -> Option<UIContainerID>;
    fn set_parent(&mut self, container_id: Option<UIContainerID>);
}