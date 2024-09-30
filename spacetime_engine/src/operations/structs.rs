use super::traits::*;

pub struct OperationQueue {
    queue: Vec<Box<dyn DynOperation>>,
}
impl OperationQueue {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, operation: Box<dyn DynOperation>) {
        self.queue.push(operation);
    }

    pub fn remove_operations(&mut self) -> Vec<Box<dyn DynOperation>> {
        self.queue.drain(..).collect()
    }
}