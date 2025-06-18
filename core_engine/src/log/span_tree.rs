use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::log::storage::LogId;

#[derive(Default)]
pub struct SpanTree {
    pub roots: RwLock<HashMap<String, Arc<RwLock<SpanNode>>>>,
}

#[derive(Default)]
pub struct SpanNode {
    pub children: RwLock<HashMap<String, Arc<RwLock<SpanNode>>>>,
    pub logs: RwLock<Vec<u64>>,
}

impl SpanTree {
    pub fn insert_log(&self, path: Vec<String>, log_id: LogId) {
        let mut current = Arc::clone(
            &self.roots
                .write()
                .unwrap()
                .entry(path[0].clone())
                .or_insert_with(|| Arc::new(RwLock::new(SpanNode::default())))
        );

        for segment in &path[1..] {
            let node = current.write().unwrap();
            let mut child_node = node.children.write().unwrap();
            let entry = child_node
                .entry(segment.clone())
                .or_insert_with(|| Arc::new(RwLock::new(SpanNode::default())));
            let new_current = Arc::clone(entry);
            drop(child_node);
            drop(node);
            current = new_current;
        }

        current.write().unwrap().logs.write().unwrap().push(log_id.0);
    }

    pub fn insert_path(&self, path: Vec<String>) {
        let mut current = Arc::clone(&self.roots.write().unwrap().entry(path[0].clone()).or_insert_with(|| Arc::new(RwLock::new(SpanNode::default()))));

        for segment in &path[1..] {
            let node = current.write().unwrap();
            let mut child_node = node.children.write().unwrap();
            let entry = child_node
                .entry(segment.clone())
                .or_insert_with(|| Arc::new(RwLock::new(SpanNode::default())));
            let new_current = Arc::clone(entry);
            drop(child_node);
            drop(node);
            current = new_current;
        }
    }
}

impl SpanTree {
    pub fn roots(&self) -> HashMap<String, Arc<RwLock<SpanNode>>> {
        self.roots.read().unwrap().clone()
    }
}
