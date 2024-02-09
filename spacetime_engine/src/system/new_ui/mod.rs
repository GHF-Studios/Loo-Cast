#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIObjectState {
    Enabled,
    Disabled
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFocusType {
    Canvas,
    Window,
    Container,
    Element,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UICanvasRenderingContext {
    ScreenSpace,
    WorldSpace,
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

pub trait UIEvent {
}

pub trait UIEventHandler {
}

pub trait UIObject {
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;

    fn on_focus(&self);
    fn on_unfocus(&self);
}

pub trait UIComponent: UIObject + UIEventHandler {
    fn get_parent_element(&self) -> Option<usize>;
    fn set_parent_element(&mut self, element_id: Option<usize>);
}

pub trait UIElement: UIObject + UIEventHandler {
    fn get_parent_container(&self) -> Option<usize>;
    fn set_parent_container(&mut self, container_id: Option<usize>);

    fn attach_component(&mut self, component: Box<dyn UIComponent>) -> usize;
    fn detach_component(&mut self, component_id: usize) -> Option<Box<dyn UIComponent>>;

    fn get_attached_component(&self, component_id: usize) -> Option<&Box<dyn UIComponent>>;
    fn get_attached_component_mut(&mut self, component_id: usize) -> Option<&mut Box<dyn UIComponent>>;

    fn get_attached_components(&self) -> Vec<&Box<dyn UIComponent>>;
    fn get_attached_components_mut(&mut self) -> Vec<&mut Box<dyn UIComponent>>;
}

pub trait UIContainer: UIObject + UIEventHandler {
    fn get_parent(&self) -> Option<(UIContainerParentType, usize)>;
    fn set_parent_container(&mut self, container_id: Option<usize>);
    fn set_parent_window(&mut self, window_id: Option<usize>);

    fn add_element(&mut self, element: Box<dyn UIElement>) -> usize;
    fn add_container(&mut self, container: Box<dyn UIContainer>);

    fn remove_element(&mut self, element_id: usize) -> Option<Box<dyn UIElement>>;
    fn remove_container(&mut self, container_id: usize) -> Option<Box<dyn UIContainer>>;

    fn get_element(&self, element_id: usize) -> Option<&Box<dyn UIElement>>;
    fn get_container(&self, container_id: usize) -> Option<&Box<dyn UIContainer>>;

    fn get_element_mut(&mut self, element_id: usize) -> Option<&mut Box<dyn UIElement>>;
    fn get_container_mut(&mut self, container_id: usize) -> Option<&mut Box<dyn UIContainer>>;

    fn get_elements(&self) -> Vec<&Box<dyn UIElement>>;
    fn get_containers(&self) -> Vec<&Box<dyn UIContainer>>;

    fn get_elements_mut(&mut self) -> Vec<&mut Box<dyn UIElement>>;
    fn get_containers_mut(&mut self) -> Vec<&mut Box<dyn UIContainer>>;

    fn focus_element(&mut self, element_id: usize);
    fn focus_container(&mut self, container_id: usize);
    fn unfocus_child(&mut self) -> (UIContainerChildType, usize);

    fn get_focused_child(&self) -> Option<(UIContainerChildType, usize)>;
    fn get_focused_child_mut(&mut self) -> Option<(UIContainerChildType, usize)>;
}

pub trait UIWindow: UIObject + UIEventHandler {
    fn get_parent_canvas(&self) -> Option<usize>;
    fn set_parent_canvas(&mut self, canvas_id: Option<usize>);
    
    fn get_size(&self) -> (f32, f32);
    fn get_position(&self) -> (f32, f32);

    fn set_size(&mut self, width: f32, height: f32);
    fn set_position(&mut self, x: f32, y: f32);

    fn add_container(&mut self, container: Box<dyn UIContainer>) -> usize;
    fn remove_container(&mut self, container_id: usize) -> Option<Box<dyn UIContainer>>;

    fn get_container(&self, container_id: usize) -> Option<&Box<dyn UIContainer>>;
    fn get_container_mut(&mut self, container_id: usize) -> Option<&mut Box<dyn UIContainer>>;

    fn get_containers(&self) -> Vec<&Box<dyn UIContainer>>;
    fn get_containers_mut(&mut self) -> Vec<&mut Box<dyn UIContainer>>;

    fn focus_container(&mut self, container_id: usize);
    fn unfocus_container(&mut self) -> usize;

    fn get_focused_container(&self) -> Option<&Box<dyn UIContainer>>;
    fn get_focused_container_mut(&mut self) -> Option<&mut Box<dyn UIContainer>>;
}

pub trait UICanvas: UIObject + UIEventHandler {
    fn get_parent_scene(&self) -> Option<usize>;
    fn set_parent_scene(&mut self, scene_id: Option<usize>);

    fn add_window(&mut self, window: Box<dyn UIWindow>) -> usize;
    fn remove_window(&mut self, window_id: usize) -> Option<Box<dyn UIWindow>>;

    fn get_window(&self, window_id: usize) -> Option<&Box<dyn UIWindow>>;
    fn get_window_mut(&mut self, window_id: usize) -> Option<&mut Box<dyn UIWindow>>;

    fn get_windows(&self) -> Vec<&Box<dyn UIWindow>>;
    fn get_windows_mut(&mut self) -> Vec<&mut Box<dyn UIWindow>>;

    fn focus_window(&mut self, window_id: usize);
    fn unfocus_window(&mut self) -> usize;

    fn get_focused_window(&self) -> Option<&Box<dyn UIWindow>>;
    fn get_focused_window_mut(&mut self) -> Option<&mut Box<dyn UIWindow>>;
}

pub trait UIScene: UIObject + UIEventHandler {
    fn add_canvas(&mut self, canvas: Box<dyn UICanvas>) -> usize;
    fn remove_canvas(&mut self, canvas_id: usize) -> Option<Box<dyn UICanvas>>;

    fn get_canvas(&self, canvas_id: usize) -> Option<&Box<dyn UICanvas>>;
    fn get_canvas_mut(&mut self, canvas_id: usize) -> Option<&mut Box<dyn UICanvas>>;

    fn get_canvases(&self) -> Vec<&Box<dyn UICanvas>>;
    fn get_canvases_mut(&mut self) -> Vec<&mut Box<dyn UICanvas>>;

    fn focus_canvas(&mut self, canvas_id: usize);
    fn unfocus_canvas(&mut self) -> usize;

    fn get_focused_canvas(&self) -> Option<&Box<dyn UICanvas>>;
    fn get_focused_canvas_mut(&mut self) -> Option<&mut Box<dyn UICanvas>>;
}

pub struct UIManager {
    current_scene: Option<Box<dyn UIScene>>,
    focused_canvas: Option<usize>,
    focused_window: Option<usize>,
    focused_container: Option<usize>,
    focused_element: Option<usize>,
}

/*
TODO: Implement Advanced UI System

1. 4-Layer Focus Management System:
    - Implement a hierarchical focus management system where focus can propagate down from Scene (non-focusable, inherently always focused) to Canvas, Window, Container, and Element layers. Components do not receive direct focus.
    - Ensure that when an object is focused, its parent (if applicable) and all ancestors up to the root are automatically focused, establishing a focused path within the hierarchy. This excludes Scenes, which are inherently focused, and Components, which are not focusable.
    - Design the system so that Elements represent the atomic unit of user interaction within the UI, with Components considered as non-focusable parts of Elements.

2. 6-Layer Event Management System with Prioritization:
    - Develop an event management system that allows events to be routed from the top of the UI hierarchy downwards towards the intended target object, specified by a unique path through the UI hierarchy.
    - Distinguish between key press events and other UI events:
        - Key Press Events System: Implement a distinct system for key press events where such events are sent to all focused objects across the four focusable layers (Canvas, Window, Container, Element) in a non-consumable manner. This ensures that key press events are universally accessible and not subject to consumption by any single UI object.
        - UI Events System: For all other types of events, implement a targeted delivery system where events are directed to a specific UI Object (or set of objects) based on their hierarchy and a specified path. This system allows events to be consumed by the target object, preventing further propagation.
    - Include functionality for EventHandler objects to consume an event, halting further propagation to children, with the mechanism for specifying the target UI Object's path within the event itself to facilitate precise routing.
    - Ensure that events can be routed accurately through the hierarchy from Scene to Canvas, Window, Container, Element, and finally to Components, unless consumed by the Element. Note that Components cannot consume events, reflecting their role as part of a larger interactive Element.

3. Implementation Details:
    - Create robust interfaces and base classes for UI objects that include methods for startup, shutdown, focusing, and event handling, adhering to the principles of the focus and event management systems described above.
    - Design the UIManager to effectively manage the current scene, focus states across different layers, and the routing and handling of events within the UI hierarchy.
    - Pay special attention to the separation of key press events from other UI events in the system's design to ensure both types of events are handled appropriately and according to their distinct characteristics.

This comprehensive plan aims to create a flexible, efficient, and intuitive UI system that enhances user interaction and event handling within the game's UI architecture.
*/

impl UIManager {
    pub fn new() -> Self {
        Self {
            current_scene: None,
            focused_canvas: None,
            focused_window: None,
            focused_container: None,
            focused_element: None,
        }
    }
}