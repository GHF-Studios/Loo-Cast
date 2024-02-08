#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIObjectState {
    Started,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderingContext {
    ScreenSpace,
    WorldSpace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusType {
    Canvas,
    Window,
    Container,
    Element,
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

pub trait UIComponent: UIObject + UIEventHandler {
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;

    fn get_parent_element(&self) -> Option<usize>;
    fn set_parent_element(&mut self, element_id: Option<usize>);
}

pub trait UIElement: UIObject + UIEventHandler {
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;
    
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
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;
    
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
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;
    
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
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;
    
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
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn get_ui_object_state(&self) -> UIObjectState;
    
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

// Implement 6-layer focus management
    // Any UIObject can be focused
    // If the UIObject is not at the top of the UI Hierarchy, it's parent will also be focused (each of the 6 layers has it's own focus)
    // A scene cannot be focused, it is always focused because it is the root of the UI Hierarchy
    // A component cannot be focused, only it's  parent element can be focused, as an element represents an atomic unit of UI (from the perspective of the user)
// Implement 6-layer event management with prioritization capabilities
    // An EventHandler can consume the event, preventing others from receiving it
    // An Event will always be passed into the UI Hierarchy from the top down

// Additional info:
    // Key press events will only be sent to focused UIObjects

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