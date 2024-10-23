use std::any::Any;
use std::collections::HashMap;
use std::num::NonZero;
use std::fmt::{Debug, Display};
use std::sync::*;
use std::hash::Hash;
use super::structs::LockingPathSegment;

#[derive(Clone)]
pub struct LockingRootNodeInfo {
    path_segment: LockingPathSegment,
    locking_state: LockingState,
    children: HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>>,
}
impl LockingRootNodeInfo {
    pub fn children(&self) -> &HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>> {
        &self.children
    }
    pub(in super) fn children_mut(&mut self) -> &mut HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>> {
        &mut self.children
    }
}
impl PartialEq for LockingRootNodeInfo {
    fn eq(&self, other: &Self) -> bool {
        for (key, _) in self.children.iter() {
            if !other.children.contains_key(key) {
                return false;
            }
        }

        true
    }
}
impl Eq for LockingRootNodeInfo {}
impl Hash for LockingRootNodeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (key, _) in self.children.iter() {
            key.hash(state);
        }
    }
}

#[derive(Clone)]
pub struct LockingBranchNodeInfo {
    path_segment: LockingPathSegment,
    locking_state: LockingState,
    parent: (LockingPathSegment, Arc<Mutex<dyn Any>>),
    children: HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>>,
}
impl LockingBranchNodeInfo {
    pub fn parent(&self) -> &(LockingPathSegment, Arc<Mutex<dyn Any>>) {
        &self.parent
    }
    pub(in super) fn parent_mut(&mut self) -> &mut (LockingPathSegment, Arc<Mutex<dyn Any>>) {
        &mut self.parent
    }
    pub fn children(&self) -> &HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>> {
        &self.children
    }
    pub(in super) fn children_mut(&mut self) -> &mut HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>> {
        &mut self.children
    }
}
impl PartialEq for LockingBranchNodeInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.parent.0 != other.parent.0 {
            return false;
        }

        for (key, _) in self.children.iter() {
            if !other.children.contains_key(key) {
                return false;
            }
        }

        true
    }
}
impl Eq for LockingBranchNodeInfo {}
impl Hash for LockingBranchNodeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.0.hash(state);

        for (key, _) in self.children.iter() {
            key.hash(state);
        }
    }
}

#[derive(Clone)]
pub struct LockingLeafNodeInfo {
    path_segment: LockingPathSegment,
    locking_state: LockingState,
    parent: (LockingPathSegment, Arc<Mutex<dyn Any>>),
}
impl LockingLeafNodeInfo {
    pub fn parent(&self) -> &(LockingPathSegment, Arc<Mutex<dyn Any>>) {
        &self.parent
    }
    pub(in super) fn parent_mut(&mut self) -> &mut (LockingPathSegment, Arc<Mutex<dyn Any>>) {
        &mut self.parent
    }
}
impl PartialEq for LockingLeafNodeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.parent.0 == other.parent.0
    }
}
impl Eq for LockingLeafNodeInfo {}
impl Hash for LockingLeafNodeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.0.hash(state);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LockingNodeInfo {
    Root(LockingRootNodeInfo),
    Branch(LockingBranchNodeInfo),
    Leaf(LockingLeafNodeInfo),
}
impl LockingNodeInfo {
    pub fn path_segment(&self) -> LockingPathSegment {
        match self {
            LockingNodeInfo::Root(info) => info.path_segment.clone(),
            LockingNodeInfo::Branch(info) => info.path_segment.clone(),
            LockingNodeInfo::Leaf(info) => info.path_segment.clone(),
        }
    }
    pub fn locking_state(&self) -> LockingState {
        match self {
            LockingNodeInfo::Root(info) => info.locking_state.clone(),
            LockingNodeInfo::Branch(info) => info.locking_state.clone(),
            LockingNodeInfo::Leaf(info) => info.locking_state.clone(),
        }
    }
}
impl Debug for LockingNodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeInfo::Root(_) => write!(f, "Root"),
            LockingNodeInfo::Branch(_) => write!(f, "Branch"),
            LockingNodeInfo::Leaf(_) => write!(f, "Leaf"),
        }
    }
}
impl Display for LockingNodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeInfo::Root(_) => write!(f, "Root"),
            LockingNodeInfo::Branch(_) => write!(f, "Branch"),
            LockingNodeInfo::Leaf(_) => write!(f, "Leaf"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockingState {
    Unlocked,
    PartiallyLocked { child_lock_count: NonZero<usize>},
    FullyLocked,
}
impl Debug for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { child_lock_count } => write!(f, "PartiallyLocked({})", child_lock_count),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}
impl Display for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { child_lock_count } => write!(f, "PartiallyLocked({})", child_lock_count),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}