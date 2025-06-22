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

    pub fn roots(&self) -> HashMap<String, Arc<RwLock<SpanNode>>> {
        self.roots.read().unwrap().clone()
    }

    /// Consumes the roots of the tree and returns two iterators:
    /// The first iterator yields (segment, node) pairs for the root nodes.
    /// The second iterator yields (segment, subtree) pairs for the subtrees rooted at each segment.
    pub fn iter(self) -> (impl Iterator<Item = (String, Arc<RwLock<SpanNode>>)>, impl Iterator<Item = (String, Self)>) {
        let roots = self.roots.read().unwrap().clone();
        let roots_iter = roots.clone().into_iter().map(|(segment, node)| (segment, node));
        let subtree_iter = roots.into_iter().map(move |(segment, node)| {
            let subtree = SpanTree {
                roots: RwLock::new(HashMap::from([(segment.clone(), node)])),
            };
            (segment, subtree)
        });

        (roots_iter, subtree_iter)
    } 
}
