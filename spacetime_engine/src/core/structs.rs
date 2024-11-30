use crate::*;
use crate::singletons::*;
use crate::core::wrappers::CoreCommandTypeRegistry;
use std::collections::HashMap;
use std::any::*;
use std::sync::{Arc, Mutex, MutexGuard};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use bevy::prelude::*;
use structs::{NumericID, StringID};
use wrappers::RootTypeRegistry;
use super::enums::*;
use super::errors::*;
use super::traits::*;

pub struct Core;
impl LockingNodeData for Core {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();
    
        let command_type_registry_path_segment = LockingPathSegment::new_string("command_types");
        let command_type_registry_path = core_path.clone().push(command_type_registry_path_segment).unwrap();
        hierarchy.insert_branch(core_path.clone(), core_mutex.clone(), command_type_registry_path_segment, CoreCommandTypeRegistry::new()).unwrap();
        hierarchy.pre_startup(core_path).unwrap();
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();

        let command_type_registry_path_segment = LockingPathSegment::new_string("command_types");
        let command_type_registry_path = core_path.clone().push(command_type_registry_path_segment).unwrap();
        hierarchy.startup::<CoreCommandTypeRegistry>(command_type_registry_path).unwrap();

        dispatch_cmd!(sync, "core.command_types.spawn_main_camera");
        dispatch_cmd!(sync, "core.command_types.spawn_start_chunks", 2);
        dispatch_cmd!(sync, "core.command_types.spawn_start_chunk_actors", 2);
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();

        let command_type_registry_path_segment = LockingPathSegment::new_string("command_types");
        let command_type_registry_path = core_path.clone().push(command_type_registry_path_segment).unwrap();
        hierarchy.post_startup::<CoreCommandTypeRegistry>(command_type_registry_path).unwrap();
    }

    fn pre_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }
}

pub enum LockingPathSegment {
    Root,
    String(StringID),
    Numeric(NumericID),
}
impl LockingPathSegment {
    pub fn new_root() -> Self {
        LockingPathSegment::Root
    }

    pub fn new_string(id: &'static str) -> Self {
        LockingPathSegment::String(StringID::new(id))
    }

    pub fn new_number(id: u64) -> Self {
        LockingPathSegment::Numeric(NumericID::new(id))
    }
}
impl Debug for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::String(id) => write!(f, "{}", id),
            LockingPathSegment::Numeric(id) => write!(f, "{}", id),
        }
    }
}
impl Display for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::String(id) => write!(f, "{}", id),
            LockingPathSegment::Numeric(id) => write!(f, "{}", id),
        }
    }
}
impl Clone for LockingPathSegment {
    fn clone(&self) -> Self {
        match self {
            LockingPathSegment::Root => LockingPathSegment::Root,
            LockingPathSegment::String(id) => LockingPathSegment::String(id.clone()),
            LockingPathSegment::Numeric(id) => LockingPathSegment::Numeric(id.clone()),
        }
    }
}
impl Copy for LockingPathSegment {}
impl PartialEq for LockingPathSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LockingPathSegment::String(id), LockingPathSegment::String(other_id)) => id == other_id,
            (LockingPathSegment::Numeric(id), LockingPathSegment::Numeric(other_id)) => id == other_id,
            _ => false,
        }
    }
}
impl Eq for LockingPathSegment {}
impl Hash for LockingPathSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LockingPathSegment::Root => "Root".hash(state),
            LockingPathSegment::String(id) => id.hash(state),
            LockingPathSegment::Numeric(id) => id.hash(state),
        }
    }
}

pub struct RelativeLockingPath {
    segments: Vec<LockingPathSegment>,
}
impl RelativeLockingPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
}
impl LockingPath for RelativeLockingPath {
    fn segments(&self) -> &Vec<LockingPathSegment> {
        &self.segments
    }
    
    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment> {
        &mut self.segments
    }

    fn push(mut self, segment: LockingPathSegment) -> Result<RelativeLockingPath, String> {
        let last_segment = self.segments.last();
        
        match last_segment {
            Some(LockingPathSegment::Root) => {
                unreachable!()
            },
            Some(LockingPathSegment::String(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::Numeric(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            None => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
        }
    }

    fn pop(mut self) -> Result<(RelativeLockingPath, LockingPathSegment), String> {
        match self.segments.pop() {
            Some(segment) => Ok((self, segment)),
            None => Err("Cannot pop segment from empty relative path!".to_string()),
        }
    }
}
impl Debug for RelativeLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Display for RelativeLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Clone for RelativeLockingPath {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}
impl PartialEq for RelativeLockingPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}
impl Eq for RelativeLockingPath {}
impl Hash for RelativeLockingPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
    }
}

pub struct AbsoluteLockingPath {
    segments: Vec<LockingPathSegment>,
}
impl AbsoluteLockingPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
    pub fn new_from_literal(path: &'static str) -> Self {
        let mut segments = Vec::new();

        for segment in path.split('.') {
            if let Ok(id) = segment.parse::<u64>() {
                segments.push(LockingPathSegment::Numeric(NumericID::new(id)));
            } else {
                segments.push(LockingPathSegment::String(StringID::new(segment)));
            }
        }

        Self {
            segments,
        }
    }
}
impl LockingPath for AbsoluteLockingPath {
    fn segments(&self) -> &Vec<LockingPathSegment> {
        &self.segments
    }

    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment> {
        &mut self.segments
    }

    fn push(mut self, segment: LockingPathSegment) -> Result<AbsoluteLockingPath, String> {
        let last_segment = self.segments.last();

        match last_segment {
            Some(LockingPathSegment::Root) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::String(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::Numeric(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            None => {
                unreachable!()
            }
        }
    }

    fn pop(mut self) -> Result<(AbsoluteLockingPath, LockingPathSegment), String> {
        if self.segments.len() == 1 {
            return Err("Cannot pop root segment from absolute path!".to_string());
        }

        match self.segments.pop() {
            Some(segment) => Ok((self, segment)),
            None => unreachable!(),
        }
    }
}
impl Debug for AbsoluteLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Display for AbsoluteLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Clone for AbsoluteLockingPath {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}
impl PartialEq for AbsoluteLockingPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}
impl Eq for AbsoluteLockingPath {}
impl Hash for AbsoluteLockingPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
    }
}



pub struct EntityInstance {
    bevy_entity_reference: Entity,
}

pub struct ComponentInstance<T: Component> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}

pub struct BundleInstance<T: Bundle> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}





pub(in super) enum LockingNodeMetadata {
    Root {
        state: LockingState,
        children: HashMap<LockingPathSegment, Arc<Mutex<LockingNode>>>,
    },
    Branch {
        path_segment: LockingPathSegment,
        state: LockingState,
        parent_type_id: TypeId,
        parent: (AbsoluteLockingPath, Arc<Mutex<LockingNode>>),
        children: HashMap<LockingPathSegment, Arc<Mutex<LockingNode>>>,
    },
    Leaf {
        path_segment: LockingPathSegment,
        state: LockingState,
        parent_type_id: TypeId,
        parent: (AbsoluteLockingPath, Arc<Mutex<LockingNode>>),
    },
}
impl LockingNodeMetadata {
    pub fn get_path_segment(&self) -> LockingPathSegment {
        match self {
            LockingNodeMetadata::Root { .. } => LockingPathSegment::Root,
            LockingNodeMetadata::Branch { path_segment, .. } => path_segment.clone(),
            LockingNodeMetadata::Leaf { path_segment, .. } => path_segment.clone(),
        }
    }

    pub fn get_state(&self) -> &LockingState {
        match self {
            LockingNodeMetadata::Root { state, .. } => state,
            LockingNodeMetadata::Branch { state, .. } => state,
            LockingNodeMetadata::Leaf { state, .. } => state,
        }
    }

    pub fn get_state_mut(&mut self) -> &mut LockingState {
        match self {
            LockingNodeMetadata::Root { state, .. } => state,
            LockingNodeMetadata::Branch { state, .. } => state,
            LockingNodeMetadata::Leaf { state, .. } => state,
        }
    }

    pub fn get_parent_type_id(&self) -> Option<TypeId> {
        match self {
            LockingNodeMetadata::Root { .. } => None,
            LockingNodeMetadata::Branch { parent_type_id, .. } => Some(parent_type_id.clone()),
            LockingNodeMetadata::Leaf { parent_type_id, .. } => Some(parent_type_id.clone()),
        }
    }

    pub fn get_parent(&self) -> Option<(AbsoluteLockingPath, Arc<Mutex<LockingNode>>)> {
        match self {
            LockingNodeMetadata::Root { .. } => None,
            LockingNodeMetadata::Branch { parent, .. } => Some(parent.clone()),
            LockingNodeMetadata::Leaf { parent, .. } => Some(parent.clone()),
        }
    }

    pub fn get_children(&self) -> Option<&HashMap<AbsoluteLockingPath, Arc<Mutex<LockingNode>>>> {
        match self {
            LockingNodeMetadata::Root { children, .. } => Some(children),
            LockingNodeMetadata::Branch { children, .. } => Some(children),
            LockingNodeMetadata::Leaf { .. } => None,
        }
    }
}
impl Debug for LockingNodeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeMetadata::Root { state, children } => {
                let children_string = children.keys().iter().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Root {{ state[ {:?} ], children[ {:?} ] }}", state, children_string)
            },
            LockingNodeMetadata::Branch { path_segment, state, parent_type_id, parent, children } => {
                let children_string = children.keys().iter().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Branch {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ], children[ {:?} ] }}", path_segment, state, parent_type_id, parent, children_string)
            },
            LockingNodeMetadata::Leaf { path_segment, state, parent_type_id, parent } => {
                write!(f, "Leaf {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ] }}", path_segment, state, parent_type_id, parent)
            },
        }
    }
}
impl Display for LockingNodeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeMetadata::Root { state, children } => {
                let children_string = children.keys().iter().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Root {{ state[ {:?} ], children[ {:?} ] }}", state, children_string)
            },
            LockingNodeMetadata::Branch { path_segment, state, parent_type_id, parent, children } => {
                let children_string = children.keys().iter().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Branch {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ], children[ {:?} ] }}", path_segment, state, parent_type_id, parent, children_string)
            },
            LockingNodeMetadata::Leaf { path_segment, state, parent_type_id, parent } => {
                write!(f, "Leaf {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ] }}", path_segment, state, parent_type_id, parent)
            },
        }
    }
}

pub(in super) struct LockingNode {
    metadata: LockingNodeMetadata,
    data: Arc<Mutex<Box<dyn Any + Send + Sync>>>,
}
impl LockingNode {
    pub fn new(metadata: LockingNodeMetadata, data: Box<dyn LockingNodeData>) -> Self {
        Self {
            metadata,
            data: Arc::new(Mutex::new(data as Box<dyn Any + Send + Sync>)),
        }
    }

    pub fn lock(&mut self) -> Result<LockedDataContainer, LockingNodeError> {
        match self.metadata.get_state() {
            LockingState::Unlocked => {},
            LockingState::PartiallyLocked { .. } => {
                return Err(LockingNodeError::AlreadyPartiallyLocked);
            },
            LockingState::FullyLocked => {
                return Err(LockingNodeError::AlreadyFullyLocked);
            },
        }

        let (parent_path, parent_mutex) = match self.metadata {
            LockingNodeMetadata::Root { .. } => {
                *self.metadata.get_state_mut() = LockingState::FullyLocked;

                let locked_children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in locked_children {
                    let child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.lock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::ChildLockError(error));
                        },
                    };
                }

                return Ok(());
            },
            LockingNodeMetadata::Branch { parent, .. } => parent,
            LockingNodeMetadata::Leaf { parent, .. } => parent,
        };

        let parent = match parent_mutex.try_lock() {
            Ok(parent) => parent,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    return Err(LockingNodeError::ParentPoisoned);
                },
                std::sync::TryLockError::WouldBlock => {
                    return Err(LockingNodeError::ParentFullyLocked);
                },
            },
        };

        match parent.metadata.get_state() {
            LockingState::FullyLocked => {
                unreachable!();
            },
            LockingState::PartiallyLocked { locked_children } => {
                if !locked_children.contains(&self.metadata.get_path_segment()) {
                    locked_children.push(self.metadata.get_path_segment());
                } else {
                    unreachable!();
                }
            },
            LockingState::Unlocked => {
                *parent.metadata.get_state_mut() = LockingState::PartiallyLocked {
                    locked_children: vec![self.metadata.get_path_segment()],
                };
            },
        }

        *self.metadata.get_state_mut() = LockingState::FullyLocked;

        let children = match self.metadata.get_children() {
            Some(children) => children,
            None => {
                return Ok(());
            },
        };
        for (child_path, child_mutex) in children {
            let child = match child_mutex.try_lock() {
                Ok(child) => child,
                Err(error) => match error {
                    std::sync::TryLockError::Poisoned(_) => {
                        return Err(LockingNodeError::ChildPoisoned);
                    },
                    std::sync::TryLockError::WouldBlock => {
                        return Err(LockingNodeError::ChildFullyLocked);
                    },
                },
            };

            match child.lock() {
                Ok(()) => {},
                Err(error) => {
                    return Err(LockingNodeError::ChildLockError(error));
                },
            };
        }

        return Ok(());
    }

    pub fn unlock(&mut self) -> Result<(), LockingNodeError> {
        match self.metadata.get_state() {
            LockingState::Unlocked => {
                return Err(LockingNodeError::AlreadyUnlocked);
            },
            LockingState::PartiallyLocked { .. } => {
                return Err(LockingNodeError::CannotUnlockPartiallyLocked);
            },
            LockingState::FullyLocked => {},
        }

        let (parent_path, parent_mutex) = match self.metadata {
            LockingNodeMetadata::Root { .. } => {
                *self.metadata.get_state_mut() = LockingState::Unlocked;

                let children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in children {
                    let child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::ChildUnlockError(error));
                        },
                    };
                }

                return Ok(());
            },
            LockingNodeMetadata::Branch { parent, .. } => parent,
            LockingNodeMetadata::Leaf { parent, .. } => parent,
        };

        let parent = match parent_mutex.try_lock() {
            Ok(parent) => parent,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    return Err(LockingNodeError::ParentPoisoned);
                },
                std::sync::TryLockError::WouldBlock => {
                    return Err(LockingNodeError::ParentFullyLocked);
                },
            },
        };

        match parent.metadata.get_state() {
            LockingState::FullyLocked => {
                unreachable!();
            },
            LockingState::PartiallyLocked { locked_children: locked_siblings } => {
                *self.metadata.get_state_mut() = LockingState::Unlocked;

                let self_path_segment = self.metadata.get_path_segment();
                locked_siblings.retain(|segment| segment != &self_path_segment);
                if locked_siblings.is_empty() {
                    match parent.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::UnlockParentError());
                        },
                    }
                }

                let children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in children {
                    let child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::ChildUnlockError(error));
                        },
                    };
                }

                return Ok(());
            },
            LockingState::Unlocked => {
                unreachable!();
            },
        }
    }
}
impl Debug for LockingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockingNode {{ metadata[ {:?} ] }}", self.metadata)
    }
}
impl Display for LockingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockingNode {{ metadata[ {:?} ] }}", self.metadata)
    }
}

pub struct LockedDataContainer {
    locked_data: Box<dyn Any + 'static>,
    is_held: bool,
}
impl LockedDataContainer {
    pub(in super) fn new<T: 'static + Send + Sync>(data: LockedData<T>) -> Self {
        Self {
            locked_data: Box::new(data),
            is_held: true,
        }
    }

    pub fn get_ref<'a, T: 'static + Send + Sync>(&'a self) -> Option<&'a MutexGuard<T>> {
        self.locked_data.downcast_ref::<LockedData<T>>().map(|data| data.get_ref())
    }

    pub fn get_mut<'a, T: 'static + Send + Sync>(&'a mut self) -> Option<&'a mut MutexGuard<T>> {
        self.locked_data.downcast_mut::<LockedData<T>>().map(|data| data.get_mut())
    }

    pub fn unlock(self) {
        self.is_held = false;
        UNLOCK_QUEUE.lock().unwrap().push(UnlockRequest {
            node_path: self.locked_data.downcast::<&LockedData>().unwrap().node_path,
        });
    }
}
impl Drop for LockedDataContainer {
    fn drop(&mut self) {
        if self.is_held {
            panic!("Locked data container was dropped without being unlocked!");
        }
    }
}

pub(in super) struct LockedData<'a, T: 'static + Send + Sync> {
    pub data_mutex: Arc<Mutex<T>>,
    pub data: MutexGuard<'a, T>,
    pub node_path: AbsoluteLockingPath,
}
impl<'a, T: 'static + Send + Sync> LockedData<'a, T> {
    pub fn new(data_mutex: Arc<Mutex<T>>, node_path: AbsoluteLockingPath) -> Self {
        Self {
            data_mutex,
            data: data_mutex.lock().unwrap(),
            node_path,
        }
    }

    pub fn get_ref(&self) -> &MutexGuard<T> {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut MutexGuard<T> {
        &mut self.data
    }
}

pub struct UnlockRequest {
    pub node_path: AbsoluteLockingPath,
}

pub struct LockingHierarchy {
    root_node: Arc<Mutex<LockingNode>>
}
impl LockingHierarchy {
    pub fn new() -> Self {
        let root_metadata = LockingNodeMetadata::Root {
            state: LockingState::Unlocked,
            children: HashMap::new(),
        };
        let root_data = Arc::new(Mutex::new(Box::new(RootTypeRegistry::new(ROOT_TYPE_BINDING))));

        Self {
            root_node: Arc::new(Mutex::new(LockingNode::new(root_metadata, root_data)))
        }
    }
    
    pub fn insert_branch<T: LockingNodeData>(&mut self, parent_path: AbsoluteLockingPath, parent_mutex: Arc<Mutex<LockingNode>>, path_segment: LockingPathSegment, data: T) -> Result<(), LockingHierarchyError> {
        todo!();
    }

    pub fn insert_leaf<T: LockingNodeData>(&mut self, parent_path: AbsoluteLockingPath, parent_mutex: Arc<Mutex<LockingNode>>, path_segment: LockingPathSegment, data: T) -> Result<(), LockingHierarchyError> {
        todo!();
    }
    
    pub fn remove<T: LockingNodeData>(&mut self, path: AbsoluteLockingPath) -> Result<T, LockingHierarchyError> {
        todo!();
    }

    pub fn pre_startup(&mut self, path: AbsoluteLockingPath) -> Result<(), LockingHierarchyError> {
        todo!();
    }

    pub fn startup(&mut self, path: AbsoluteLockingPath) -> Result<(), LockingHierarchyError> {
        todo!();
    }

    pub fn post_startup(&mut self, path: AbsoluteLockingPath) -> Result<(), LockingHierarchyError> {
        todo!();
    }

    pub fn contains(&self, path: AbsoluteLockingPath) -> bool {
        todo!();
    }

    pub fn is<T: LockingNodeData>(&self, path: AbsoluteLockingPath) -> bool {
        todo!();
    }

    pub fn get<T: LockingNodeData>(&self, path: AbsoluteLockingPath) -> Result<&T, LockingHierarchyError> {
        todo!();
    }

    pub fn get_mut<T: LockingNodeData>(&mut self, path: AbsoluteLockingPath) -> Result<&mut T, LockingHierarchyError> {
        todo!();
    }
    
    pub fn get_node_raw(&self, path: AbsoluteLockingPath) -> Result<Arc<Mutex<LockingNode>>, LockingHierarchyError> {
        todo!();
    }

    pub fn get_node(&self, path: AbsoluteLockingPath) -> Result<&LockingNode, LockingHierarchyError> {
        todo!();
    }

    pub fn get_node_mut(&mut self, path: AbsoluteLockingPath) -> Result<&mut LockingNode, LockingHierarchyError> {
        todo!();
    }
}
impl Debug for LockingHierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_node = match self.root_node.try_lock() {
            Ok(root_node) => root_node,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Root node mutex is poisoned!");
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Root node mutex is locked!");
                },
            },
        };

        write!(f, "LockingHierarchy {{ root_node[ {:?} ] }}", root_node)
    }
}
impl Display for LockingHierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_node = match self.root_node.try_lock() {
            Ok(root_node) => root_node,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Root node mutex is poisoned!");
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Root node mutex is locked!");
                },
            },
        };

        write!(f, "LockingHierarchy {{ root_node[ {:?} ] }}", root_node)
    }
}
