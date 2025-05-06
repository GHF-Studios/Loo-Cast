use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;

#[derive(Default)]
pub struct WorkflowContext {
    map: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl WorkflowContext {
    pub fn insert<T: 'static + Send>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get_mut<T: 'static + Send>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

static CONTEXTS: Lazy<DashMap<Uuid, Arc<Mutex<WorkflowContext>>>> =
    Lazy::new(DashMap::new);

tokio::task_local! {
    static CURRENT_WORKFLOW_ID: Uuid;
}

pub fn set_context<T: 'static + Send>(val: T) {
    let id = CURRENT_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS.get(&id).expect("Missing workflow context").clone();
    let mut ctx = ctx.lock().expect("Workflow context mutex poisoned");
    (*ctx).insert::<Option<T>>(Some(val));
}

pub fn get_context<T: 'static + Send>() -> T {
    let id = CURRENT_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS.get(&id).expect("Missing workflow context").clone();
    let mut ctx = ctx.lock().expect("Workflow context mutex poisoned");
    (*ctx).get_mut::<Option<T>>()
        .expect("Context value not found")
        .take()
        .expect("Context value was empty")
}

pub fn clear_all_context(id: Uuid) {
    CONTEXTS.remove(&id);
}

pub struct ScopedCompositeWorkflowContext {
    pub id: Uuid,
}

impl ScopedCompositeWorkflowContext {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        CONTEXTS.insert(id, Arc::new(Mutex::new(WorkflowContext::default())));
        Self { id }
    }

    pub async fn run<F, Fut>(self, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let id = self.id;
        CURRENT_WORKFLOW_ID.scope(id, async {
            f().await;
            clear_all_context(id);
        }).await;
    }

    pub async fn run_fallible<F, Fut, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), E>>,
    {
        let id = self.id;
        CURRENT_WORKFLOW_ID.scope(id, async {
            let result = f().await;
            clear_all_context(id);
            result
        }).await
    }
}
