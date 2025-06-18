use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::log::storage::LogId;
use crate::log::types::LocationPathSegment;

#[derive(Default)]
pub struct LocationTree {
    pub roots: RwLock<HashMap<LocationPathSegment, Arc<RwLock<LocationNode>>>>,
}

#[derive(Default)]
pub struct LocationNode {
    pub children: RwLock<HashMap<LocationPathSegment, Arc<RwLock<LocationNode>>>>,
    pub logs: RwLock<Vec<u64>>,
}

impl LocationTree {
    pub fn insert_log(&self, path: Vec<LocationPathSegment>, log_id: LogId) {
        let mut current = Arc::clone(
            &self.roots
                .write()
                .unwrap()
                .entry(path[0].clone())
                .or_insert_with(|| Arc::new(RwLock::new(LocationNode::default()))),
        );

        for segment in &path[1..] {
            let node = current.write().unwrap();
            let mut child_node = node.children.write().unwrap();
            let entry = child_node
                .entry(segment.clone())
                .or_insert_with(|| Arc::new(RwLock::new(LocationNode::default())));
            let new_current = Arc::clone(entry);
            drop(child_node);
            drop(node);
            current = new_current;
        }

        current.write().unwrap().logs.write().unwrap().push(log_id.0);
    }

    pub fn insert_path(&self, path: Vec<LocationPathSegment>) {
        let mut current = Arc::clone(
            &self.roots
                .write()
                .unwrap()
                .entry(path[0].clone())
                .or_insert_with(|| Arc::new(RwLock::new(LocationNode::default()))),
        );

        for segment in &path[1..] {
            let node = current.write().unwrap();
            let mut child_node = node.children.write().unwrap();
            let entry = child_node
                .entry(segment.clone())
                .or_insert_with(|| Arc::new(RwLock::new(LocationNode::default())));
            let new_current = Arc::clone(entry);
            drop(child_node);
            drop(node);
            current = new_current;
        }
    }
}

impl LocationTree {
    pub fn roots(&self) -> HashMap<LocationPathSegment, Arc<RwLock<LocationNode>>> {
        self.roots.read().unwrap().clone()
    }
}
