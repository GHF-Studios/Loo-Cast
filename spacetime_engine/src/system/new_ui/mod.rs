use std::{any::TypeId, collections::HashMap, collections::HashSet};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIObjectDisableError {
    AlreadyDisabled,
    ParentEnabled,
}

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

pub trait UIContainer: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UIContainerID>;
    fn set_id(&mut self, container_id: Option<UIContainerID>);

    fn get_parent(&self) -> Option<(UIContainerParentType, usize)>;
    fn set_parent_container(&mut self, container_id: Option<UIContainerID>);
    fn set_parent_window(&mut self, window_id: Option<UIWindowID>);

    fn add_element(&mut self, element: Box<dyn UIElement>) -> UIElementID;
    fn add_container(&mut self, container: Box<dyn UIContainer>);

    fn remove_element(&mut self, element_id: UIElementID) -> Option<Box<dyn UIElement>>;
    fn remove_container(&mut self, container_id: UIContainerID) -> Option<Box<dyn UIContainer>>;

    fn get_element(&self, element_id: UIElementID) -> Option<&dyn UIElement>;
    fn get_container(&self, container_id: UIContainerID) -> Option<&dyn UIContainer>;

    fn get_elements(&self) -> Vec<&dyn UIElement>;
    fn get_containers(&self) -> Vec<&dyn UIContainer>;

    fn focus_element(&mut self, element_id: UIElementID);
    fn focus_container(&mut self, container_id: UIContainerID);

    fn unfocus_child(&mut self) -> (UIContainerChildType, usize);

    fn get_focused_child(&self) -> Option<(UIContainerChildType, usize)>;
}

type UIWindowID = usize;

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

    fn add_container(&mut self, container: Box<dyn UIContainer>) -> UIContainerID;
    fn remove_container(&mut self, container_id: UIContainerID) -> Option<Box<dyn UIContainer>>;

    fn get_container(&self, container_id: UIContainerID) -> Option<&dyn UIContainer>;
    fn get_containers(&self) -> Vec<&dyn UIContainer>;

    fn focus_container(&mut self, container_id: UIContainerID);
    fn unfocus_container(&mut self) -> UIContainerID;

    fn get_focused_container(&self) -> Option<&dyn UIContainer>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UICanvasRenderingContext {
    ScreenSpace,
    WorldSpace,
}

type UICanvasID = usize;

pub trait UICanvas: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UICanvasID>;
    fn set_id(&mut self, canvas_id: Option<UICanvasID>);

    fn get_parent(&self) -> Option<UISceneID>;
    fn set_parent(&mut self, scene_id: Option<UISceneID>);

    fn add_window(&mut self, window: Box<dyn UIWindow>) -> UIWindowID;
    fn remove_window(&mut self, window_id: UIWindowID) -> Option<Box<dyn UIWindow>>;

    fn get_window(&self, window_id: UIWindowID) -> Option<&dyn UIWindow>;
    fn get_windows(&self) -> Vec<&dyn UIWindow>;

    fn focus_window(&mut self, window_id: UIWindowID);
    fn unfocus_window(&mut self) -> UIWindowID;

    fn get_focused_window(&self) -> Option<&dyn UIWindow>;
}

pub type UISceneID = usize;

pub trait UIScene: UIObject {
    fn get_type_id(&self) -> TypeId;
    fn get_id(&self) -> Option<UISceneID>;
    fn set_id(&mut self, scene_id: Option<UISceneID>);

    fn add_canvas(&mut self, canvas: &mut Box<dyn UICanvas>) -> UICanvasID;
    fn remove_canvas(&mut self, canvas_id: UICanvasID) -> Option<Box<dyn UICanvas>>;

    fn get_canvas(&self, canvas_id: UICanvasID) -> Option<&dyn UICanvas>;
    fn get_canvases(&self) -> Vec<&dyn UICanvas>;

    fn focus_canvas(&mut self, canvas_id: UICanvasID);
    fn unfocus_canvas(&mut self) -> UICanvasID;

    fn get_focused_canvas(&self) -> Option<&dyn UICanvas>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterSceneTypeError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterCanvasTypeError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterWindowTypeError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterContainerTypeError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterElementTypeError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterSceneTypeError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterCanvasTypeError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterWindowTypeError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterContainerTypeError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterElementTypeError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterSceneError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterCanvasError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterWindowError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterContainerError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRegisterElementError {
    AlreadyRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterSceneError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterCanvasError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterWindowError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterContainerError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnregisterElementError {
    AlreadyUnregistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusSceneError {
    AlreadyFocused,
    NotRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusCanvasError {
    AlreadyFocused,
    NotRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusWindowError {
    AlreadyFocused,
    NotRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusContainerError {
    AlreadyFocused,
    NotRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerFocusElementError {
    AlreadyFocused,
    NotRegistered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusSceneError {
    AlreadyUnfocused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusCanvasError {
    AlreadyUnfocused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusWindowError {
    AlreadyUnfocused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusContainerError {
    AlreadyUnfocused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerUnfocusElementError {
    AlreadyUnfocused,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedSceneIDError {
    AllIDsInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleSceneIDError {
    AlreadyRecycled,
    StillInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedCanvasIDError {
    AllIDsInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleCanvasIDError {
    AlreadyRecycled,
    StillInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedWindowIDError {
    AllIDsInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleWindowIDError {
    AlreadyRecycled,
    StillInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedContainerIDError {
    AllIDsInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleContainerIDError {
    AlreadyRecycled,
    StillInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerGetUnusedElementIDError {
    AllIDsInUse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIManagerRecycleElementIDError {
    AlreadyRecycled,
    StillInUse,
}

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
    
    registered_scenes: HashMap<UISceneID, Box<dyn UIScene>>,
    registered_canvases: HashMap<UICanvasID, Box<dyn UICanvas>>,
    registered_windows: HashMap<UIWindowID, Box<dyn UIWindow>>,
    registered_containers: HashMap<UIContainerID, Box<dyn UIContainer>>,
    registered_elements: HashMap<UIElementID, Box<dyn UIElement>>,

    focused_scene: Option<UISceneID>,
    focused_canvas: Option<UICanvasID>,
    focused_window: Option<UIWindowID>,
    focused_container: Option<UIContainerID>,
    focused_element: Option<UIElementID>,
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

    pub fn get_unused_scene_id(&mut self) -> Result<UISceneID, UIManagerGetUnusedSceneIDError> {
        let unused_scene_id = match self.recycled_scene_ids.pop() {
            Some(recycled_scene_id) => recycled_scene_id,
            None => {
                let new_scene_id = self.new_scene_id;

                self.new_scene_id += 1;

                new_scene_id
            },
        };

        Ok(unused_scene_id)
    }

    pub fn recycle_scene_id(&mut self, scene_id: UISceneID) -> Result<(), UIManagerRecycleSceneIDError> {
        if self.recycled_scene_ids.contains(&scene_id) {
            return Err(UIManagerRecycleSceneIDError::AlreadyRecycled);
        }

        if self.registered_scenes.contains_key(&scene_id) {
            return Err(UIManagerRecycleSceneIDError::StillInUse);
        }

        self.recycled_scene_ids.push(scene_id);

        Ok(())
    }

    pub fn get_unused_canvas_id(&mut self) -> Result<UICanvasID, UIManagerGetUnusedCanvasIDError> {
        let unused_canvas_id = match self.recycled_canvas_ids.pop() {
            Some(recycled_canvas_id) => recycled_canvas_id,
            None => {
                let new_canvas_id = self.new_canvas_id;

                self.new_canvas_id += 1;

                new_canvas_id
            },
        };

        Ok(unused_canvas_id)
    }

    pub fn recycle_canvas_id(&mut self, canvas_id: UICanvasID) -> Result<(), UIManagerRecycleCanvasIDError> {
        if self.recycled_canvas_ids.contains(&canvas_id) {
            return Err(UIManagerRecycleCanvasIDError::AlreadyRecycled);
        }

        if self.registered_canvases.contains_key(&canvas_id) {
            return Err(UIManagerRecycleCanvasIDError::StillInUse);
        }

        self.recycled_canvas_ids.push(canvas_id);

        Ok(())
    }

    pub fn get_unused_window_id(&mut self) -> Result<UIWindowID, UIManagerGetUnusedWindowIDError> {
        let unused_window_id = match self.recycled_window_ids.pop() {
            Some(recycled_window_id) => recycled_window_id,
            None => {
                let new_window_id = self.new_window_id;

                self.new_window_id += 1;

                new_window_id
            },
        };

        Ok(unused_window_id)
    }

    pub fn recycle_window_id(&mut self, window_id: UIWindowID) -> Result<(), UIManagerRecycleWindowIDError> {
        if self.recycled_window_ids.contains(&window_id) {
            return Err(UIManagerRecycleWindowIDError::AlreadyRecycled);
        }

        if self.registered_windows.contains_key(&window_id) {
            return Err(UIManagerRecycleWindowIDError::StillInUse);
        }

        self.recycled_window_ids.push(window_id);

        Ok(())
    }

    pub fn get_unused_container_id(&mut self) -> Result<UIContainerID, UIManagerGetUnusedContainerIDError> {
        let unused_container_id = match self.recycled_container_ids.pop() {
            Some(recycled_container_id) => recycled_container_id,
            None => {
                let new_container_id = self.new_container_id;

                self.new_container_id += 1;

                new_container_id
            },
        };

        Ok(unused_container_id)
    }

    pub fn recycle_container_id(&mut self, container_id: UIContainerID) -> Result<(), UIManagerRecycleContainerIDError> {
        if self.recycled_container_ids.contains(&container_id) {
            return Err(UIManagerRecycleContainerIDError::AlreadyRecycled);
        }

        if self.registered_containers.contains_key(&container_id) {
            return Err(UIManagerRecycleContainerIDError::StillInUse);
        }

        self.recycled_container_ids.push(container_id);

        Ok(())
    }

    pub fn get_unused_element_id(&mut self) -> Result<UIElementID, UIManagerGetUnusedElementIDError> {
        let unused_element_id = match self.recycled_element_ids.pop() {
            Some(recycled_element_id) => recycled_element_id,
            None => {
                let new_element_id = self.new_element_id;

                self.new_element_id += 1;

                new_element_id
            },
        };

        Ok(unused_element_id)
    }

    pub fn recycle_element_id(&mut self, element_id: UIElementID) -> Result<(), UIManagerRecycleElementIDError> {
        if self.recycled_element_ids.contains(&element_id) {
            return Err(UIManagerRecycleElementIDError::AlreadyRecycled);
        }

        if self.registered_elements.contains_key(&element_id) {
            return Err(UIManagerRecycleElementIDError::StillInUse);
        }

        self.recycled_element_ids.push(element_id);

        Ok(())
    }

    pub fn register_scene<T: 'static + UIScene>(&mut self, mut scene: Box<T>) -> Result<UISceneID, UIManagerRegisterSceneError> {
        if scene.get_id().is_some() {
            return Err(UIManagerRegisterSceneError::AlreadyRegistered);
        }

        let scene_id = match self.get_unused_scene_id() {
            Ok(unused_scene_id) => {
                scene.set_id(Some(unused_scene_id));

                unused_scene_id
            },
            Err(UIManagerGetUnusedSceneIDError::AllIDsInUse) => return Err(UIManagerRegisterSceneError::AlreadyRegistered),
        };

        if self.registered_scenes.insert(scene_id, scene).is_none() {
            Ok(scene_id)
        } else {
            Err(UIManagerRegisterSceneError::AlreadyRegistered)
        }
    }

    pub fn register_canvas<T: 'static + UICanvas>(&mut self, mut canvas: Box<T>) -> Result<UICanvasID, UIManagerRegisterCanvasError> {
        if canvas.get_id().is_some() {
            return Err(UIManagerRegisterCanvasError::AlreadyRegistered);
        }

        let canvas_id = match self.get_unused_canvas_id() {
            Ok(unused_canvas_id) => {
                canvas.set_id(Some(unused_canvas_id));

                unused_canvas_id
            },
            Err(UIManagerGetUnusedCanvasIDError::AllIDsInUse) => return Err(UIManagerRegisterCanvasError::AlreadyRegistered),
        };

        if self.registered_canvases.insert(canvas_id, canvas).is_none() {
            Ok(canvas_id)
        } else {
            Err(UIManagerRegisterCanvasError::AlreadyRegistered)
        }
    }

    pub fn register_window<T: 'static + UIWindow>(&mut self, mut window: Box<T>) -> Result<UIWindowID, UIManagerRegisterWindowError> {
        if window.get_id().is_some() {
            return Err(UIManagerRegisterWindowError::AlreadyRegistered);
        }

        let window_id = match self.get_unused_window_id() {
            Ok(unused_window_id) => {
                window.set_id(Some(unused_window_id));

                unused_window_id
            },
            Err(UIManagerGetUnusedWindowIDError::AllIDsInUse) => return Err(UIManagerRegisterWindowError::AlreadyRegistered),
        };

        if self.registered_windows.insert(window_id, window).is_none() {
            Ok(window_id)
        } else {
            Err(UIManagerRegisterWindowError::AlreadyRegistered)
        }
    }

    pub fn register_container<T: 'static + UIContainer>(&mut self, mut container: Box<T>) -> Result<UIContainerID, UIManagerRegisterContainerError> {
        if container.get_id().is_some() {
            return Err(UIManagerRegisterContainerError::AlreadyRegistered);
        }

        let container_id = match self.get_unused_container_id() {
            Ok(unused_container_id) => {
                container.set_id(Some(unused_container_id));

                unused_container_id
            },
            Err(UIManagerGetUnusedContainerIDError::AllIDsInUse) => return Err(UIManagerRegisterContainerError::AlreadyRegistered),
        };

        if self.registered_containers.insert(container_id, container).is_none() {
            Ok(container_id)
        } else {
            Err(UIManagerRegisterContainerError::AlreadyRegistered)
        }
    }

    pub fn register_element<T: 'static + UIElement>(&mut self, mut element: Box<T>) -> Result<UIElementID, UIManagerRegisterElementError> {
        if element.get_id().is_some() {
            return Err(UIManagerRegisterElementError::AlreadyRegistered);
        }

        let element_id = match self.get_unused_element_id() {
            Ok(unused_element_id) => {
                element.set_id(Some(unused_element_id));

                unused_element_id
            },
            Err(UIManagerGetUnusedElementIDError::AllIDsInUse) => return Err(UIManagerRegisterElementError::AlreadyRegistered),
        };

        if self.registered_elements.insert(element_id, element).is_none() {
            Ok(element_id)
        } else {
            Err(UIManagerRegisterElementError::AlreadyRegistered)
        }
    }

    pub fn unregister_scene(&mut self, scene_id: UISceneID) -> Result<(), UIManagerUnregisterSceneError> {
        match self.registered_scenes.remove(&scene_id) {
            Some(mut scene) => scene.set_id(None),
            None => {
                return Err(UIManagerUnregisterSceneError::AlreadyUnregistered);
            }
        }

        match self.recycle_scene_id(scene_id) {
            Ok(_) => Ok(()),
            Err(UIManagerRecycleSceneIDError::AlreadyRecycled) => panic!("Scene ID is already recycled!"),
            Err(UIManagerRecycleSceneIDError::StillInUse) => panic!("Scene ID is still in use!"),
        }
    }

    pub fn unregister_canvas(&mut self, canvas_id: UICanvasID) -> Result<(), UIManagerUnregisterCanvasError> {
        match self.registered_canvases.remove(&canvas_id) {
            Some(mut canvas) => canvas.set_id(None),
            None => {
                return Err(UIManagerUnregisterCanvasError::AlreadyUnregistered);
            }
        }

        match self.recycle_canvas_id(canvas_id) {
            Ok(_) => Ok(()),
            Err(UIManagerRecycleCanvasIDError::AlreadyRecycled) => panic!("Canvas ID is already recycled!"),
            Err(UIManagerRecycleCanvasIDError::StillInUse) => panic!("Canvas ID is still in use!"),
        }
    }

    pub fn unregister_window(&mut self, window_id: UIWindowID) -> Result<(), UIManagerUnregisterWindowError> {
        match self.registered_windows.remove(&window_id) {
            Some(mut window) => window.set_id(None),
            None => {
                return Err(UIManagerUnregisterWindowError::AlreadyUnregistered);
            }
        }

        match self.recycle_window_id(window_id) {
            Ok(_) => Ok(()),
            Err(UIManagerRecycleWindowIDError::AlreadyRecycled) => panic!("Window ID is already recycled!"),
            Err(UIManagerRecycleWindowIDError::StillInUse) => panic!("Window ID is still in use!"),
        }
    }

    pub fn unregister_container(&mut self, container_id: UIContainerID) -> Result<(), UIManagerUnregisterContainerError> {
        match self.registered_containers.remove(&container_id) {
            Some(mut container) => container.set_id(None),
            None => {
                return Err(UIManagerUnregisterContainerError::AlreadyUnregistered);
            }
        }

        match self.recycle_container_id(container_id) {
            Ok(_) => Ok(()),
            Err(UIManagerRecycleContainerIDError::AlreadyRecycled) => panic!("Container ID is already recycled!"),
            Err(UIManagerRecycleContainerIDError::StillInUse) => panic!("Container ID is still in use!"),
        }
    }

    pub fn unregister_element(&mut self, element_id: UIElementID) -> Result<(), UIManagerUnregisterElementError> {
        match self.registered_elements.remove(&element_id) {
            Some(mut element) => element.set_id(None),
            None => {
                return Err(UIManagerUnregisterElementError::AlreadyUnregistered);
            }
        }

        match self.recycle_element_id(element_id) {
            Ok(_) => Ok(()),
            Err(UIManagerRecycleElementIDError::AlreadyRecycled) => panic!("Element ID is already recycled!"),
            Err(UIManagerRecycleElementIDError::StillInUse) => panic!("Element ID is still in use!"),
        }
    }

    pub fn is_scene_registered(&self, scene_id: UISceneID) -> bool {
        self.registered_scenes.contains_key(&scene_id)
    }

    pub fn is_canvas_registered(&self, canvas_id: UICanvasID) -> bool {
        self.registered_canvases.contains_key(&canvas_id)
    }

    pub fn is_window_registered(&self, window_id: UIWindowID) -> bool {
        self.registered_windows.contains_key(&window_id)
    }

    pub fn is_container_registered(&self, container_id: UIContainerID) -> bool {
        self.registered_containers.contains_key(&container_id)
    }

    pub fn is_element_registered(&self, element_id: UIElementID) -> bool {
        self.registered_elements.contains_key(&element_id)
    }

    pub fn focus_scene(&mut self, scene_id: UISceneID) -> Result<(), UIManagerFocusSceneError> {
        if self.focused_scene.is_some() {
            return Err(UIManagerFocusSceneError::AlreadyFocused);
        }

        if !self.registered_scenes.contains_key(&scene_id) {
            return Err(UIManagerFocusSceneError::NotRegistered);
        }

        self.focused_scene = Some(scene_id);

        Ok(())
    }

    pub fn focus_canvas(&mut self, canvas_id: UICanvasID) -> Result<(), UIManagerFocusCanvasError> {
        if self.focused_canvas.is_some() {
            return Err(UIManagerFocusCanvasError::AlreadyFocused);
        }

        if !self.registered_canvases.contains_key(&canvas_id) {
            return Err(UIManagerFocusCanvasError::NotRegistered);
        }

        self.focused_canvas = Some(canvas_id);

        Ok(())
    }

    pub fn focus_window(&mut self, window_id: UIWindowID) -> Result<(), UIManagerFocusWindowError> {
        if self.focused_window.is_some() {
            return Err(UIManagerFocusWindowError::AlreadyFocused);
        }

        if !self.registered_windows.contains_key(&window_id) {
            return Err(UIManagerFocusWindowError::NotRegistered);
        }

        self.focused_window = Some(window_id);

        Ok(())
    }

    pub fn focus_container(&mut self, container_id: UIContainerID) -> Result<(), UIManagerFocusContainerError> {
        if self.focused_container.is_some() {
            return Err(UIManagerFocusContainerError::AlreadyFocused);
        }

        if !self.registered_containers.contains_key(&container_id) {
            return Err(UIManagerFocusContainerError::NotRegistered);
        }

        self.focused_container = Some(container_id);

        Ok(())
    }

    pub fn focus_element(&mut self, element_id: UIElementID) -> Result<(), UIManagerFocusElementError> {
        if self.focused_element.is_some() {
            return Err(UIManagerFocusElementError::AlreadyFocused);
        }

        if !self.registered_elements.contains_key(&element_id) {
            return Err(UIManagerFocusElementError::NotRegistered);
        }

        self.focused_element = Some(element_id);

        Ok(())
    }

    pub fn unfocus_scene(&mut self) -> Result<(), UIManagerUnfocusSceneError> {
        if self.focused_scene.is_none() {
            return Err(UIManagerUnfocusSceneError::AlreadyUnfocused);
        }

        self.focused_scene = None;

        Ok(())
    }

    pub fn unfocus_canvas(&mut self) -> Result<(), UIManagerUnfocusCanvasError> {
        if self.focused_canvas.is_none() {
            return Err(UIManagerUnfocusCanvasError::AlreadyUnfocused);
        }

        self.focused_canvas = None;

        Ok(())
    }

    pub fn unfocus_window(&mut self) -> Result<(), UIManagerUnfocusWindowError> {
        if self.focused_window.is_none() {
            return Err(UIManagerUnfocusWindowError::AlreadyUnfocused);
        }

        self.focused_window = None;

        Ok(())
    }

    pub fn unfocus_container(&mut self) -> Result<(), UIManagerUnfocusContainerError> {
        if self.focused_container.is_none() {
            return Err(UIManagerUnfocusContainerError::AlreadyUnfocused);
        }

        self.focused_container = None;

        Ok(())
    }

    pub fn unfocus_element(&mut self) -> Result<(), UIManagerUnfocusElementError> {
        if self.focused_element.is_none() {
            return Err(UIManagerUnfocusElementError::AlreadyUnfocused);
        }

        self.focused_element = None;

        Ok(())
    }

    pub fn is_scene_focused(&self, scene_id: UISceneID) -> bool {
        self.focused_scene == Some(scene_id)
    }

    pub fn is_canvas_focused(&self, canvas_id: UICanvasID) -> bool {
        self.focused_canvas == Some(canvas_id)
    }

    pub fn is_window_focused(&self, window_id: UIWindowID) -> bool {
        self.focused_window == Some(window_id)
    }

    pub fn is_container_focused(&self, container_id: UIContainerID) -> bool {
        self.focused_container == Some(container_id)
    }

    pub fn is_element_focused(&self, element_id: UIElementID) -> bool {
        self.focused_element == Some(element_id)
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_focused_scene(&self) -> Option<&Box<dyn UIScene>> {
        self.focused_scene.and_then(|scene_id| self.registered_scenes.get(&scene_id))
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_focused_canvas(&self) -> Option<&Box<dyn UICanvas>> {
        self.focused_canvas.and_then(|canvas_id| self.registered_canvases.get(&canvas_id))
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_focused_window(&self) -> Option<&Box<dyn UIWindow>> {
        self.focused_window.and_then(|window_id| self.registered_windows.get(&window_id))
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_focused_container(&self) -> Option<&Box<dyn UIContainer>> {
        self.focused_container.and_then(|container_id| self.registered_containers.get(&container_id))
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_focused_element(&self) -> Option<&Box<dyn UIElement>> {
        self.focused_element.and_then(|element_id| self.registered_elements.get(&element_id))
    }

    pub fn get_focused_scene_mut(&mut self) -> Option<&mut Box<dyn UIScene>> {
        self.focused_scene.and_then(move |scene_id| self.registered_scenes.get_mut(&scene_id))
    }

    pub fn get_focused_canvas_mut(&mut self) -> Option<&mut Box<dyn UICanvas>> {
        self.focused_canvas.and_then(move |canvas_id| self.registered_canvases.get_mut(&canvas_id))
    }

    pub fn get_focused_window_mut(&mut self) -> Option<&mut Box<dyn UIWindow>> {
        self.focused_window.and_then(move |window_id| self.registered_windows.get_mut(&window_id))
    }

    pub fn get_focused_container_mut(&mut self) -> Option<&mut Box<dyn UIContainer>> {
        self.focused_container.and_then(move |container_id| self.registered_containers.get_mut(&container_id))
    }

    pub fn get_focused_element_mut(&mut self) -> Option<&mut Box<dyn UIElement>> {
        self.focused_element.and_then(move |element_id| self.registered_elements.get_mut(&element_id))
    }
}

/*
TODO: Implement Advanced UI System (RAW USER CREATED)

1. 4-Layer Focus Management
    - Focus Hierarchy: I've designed a focus management system that respects the hierarchical structure of the UI. Each layer in the hierarchy (Canvas, Window, Container, Element) can have its focus, with the focus state propagating down the hierarchy but not up to the root (Scene) or down to the atomic components.
    - Automatic Parent Focus: When an object receives focus, its parent in the hierarchy (if applicable) automatically gains focus as well, ensuring that the entire path from the focused object up to the root is in a focused state. This excludes Scenes, which are inherently focused as the root, and Components, which cannot be focused directly.
    - Focus Constraints: Components are considered part of their parent Element and do not receive direct focus. This reflects a design choice that treats Elements as the smallest user-interactable unit. Scenes are always in focus by design, simplifying focus management at the top level.
2. 6-Layer Event Management with Prioritization
    - Event Propagation: Events are passed down from the top of the UI hierarchy (Scene) to the targeted UI Object, with the ability for any EventHandler to consume the event, thus preventing it from propagating further. This allows for precise control over how events are handled and ensures that only the most relevant object processes the event.
    - Event Handling Layers: The system distinguishes between key press events, which are sent to all focused objects across the four focusable layers, and other types of events, which target specific UI Objects based on their position in the hierarchy and the event's specified target path.
    - Event Targeting: Non-key press events require specifying the intended target UI Object's "String" (a path through the UI hierarchy), enabling the UIManager to route the event through the hierarchy (Scene > Canvas > Window > Container > Element > Component) to the specified target. This mechanism ensures that events are directed accurately to their intended recipients.
    - Event Consumption: Elements have the ability to consume events, preventing further propagation to their components. However, components cannot consume events, as they are not directly focusable. This maintains a clear separation between Elements (which can interact with and potentially halt events) and Components (which can react to events but not stop their propagation).

+
3. Implementation Details
    "Event Handling Layers: The system distinguishes between key press events, which are sent to all focused objects across the four focusable layers, and other types of events, which target specific UI Objects based on their position in the hierarchy and the event's specified target path."
    To be precise: The key press events are an entirely different system from the ui events, because they are sent in a completely different, unconsumable way.
*/

/*
TODO: Implement Advanced UI System (AI "enhanced")

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