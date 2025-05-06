use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;

#[derive(Hash, Eq, PartialEq)]
struct ContextKey {
    type_id: TypeId,
    name: &'static str,
}
impl ContextKey {
    fn new<T: 'static + Send>(name: &'static str) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            name,
        }
    }
}

#[derive(Default)]
pub struct CompositeWorkflowContext {
    map: HashMap<ContextKey, Box<dyn Any + Send>>,
}

static CONTEXTS: Lazy<DashMap<Uuid, Arc<Mutex<CompositeWorkflowContext>>>> =
    Lazy::new(DashMap::new);

tokio::task_local! {
    pub static CURRENT_COMPOSITE_WORKFLOW_ID: Uuid;
}

pub fn set_context<T: 'static + Send>(name: &'static str, val: T) {
    let id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS.get(&id).expect("Missing workflow context").clone();
    let mut ctx = ctx.lock().expect("Workflow context mutex poisoned");
    ctx.map.insert(
        ContextKey {
            type_id: TypeId::of::<T>(),
            name,
        },
        Box::new(Some(val)),
    );
}

pub fn get_context<T: 'static + Send>(name: &'static str) -> T {
    let id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS.get(&id).expect("Missing workflow context").clone();
    let mut ctx = ctx.lock().expect("Workflow context mutex poisoned");

    ctx.map
        .get_mut(&ContextKey {
            type_id: TypeId::of::<T>(),
            name,
        })
        .expect("Context value not found")
        .downcast_mut::<Option<T>>()
        .expect("Context type mismatch")
        .take()
        .expect("Context value was empty")
}

pub fn clear_all_context(id: Uuid) {
    CONTEXTS.remove(&id);
}

pub struct ScopedCompositeWorkflowContext {
    pub id: Uuid,
    pub returns: Arc<Mutex<HashMap<String, Box<dyn Any + Send>>>>,
}

impl ScopedCompositeWorkflowContext {
    fn new() -> Self {
        let id: Uuid = Uuid::new_v4();
        let returns = Arc::new(Mutex::new(HashMap::new()));
        CONTEXTS.insert(id, Arc::new(Mutex::new(CompositeWorkflowContext::default())));
        Self { id, returns }
    }

    pub async fn run<F, Fut>(self, f: F) -> ScopedCompositeWorkflowContext
    where
        F: FnOnce(Self) -> Fut,
        Fut: std::future::Future<Output = ScopedCompositeWorkflowContext>,
    {
        let id = self.id;
        CURRENT_COMPOSITE_WORKFLOW_ID.scope(id, async {
            let ctx = f(self).await;
            clear_all_context(id);
            ctx
        }).await
    }

    pub async fn run_fallible<F, Fut, E>(self, f: F) -> (ScopedCompositeWorkflowContext, Result<(), E>)
    where
        F: FnOnce(Self) -> Fut,
        Fut: std::future::Future<Output = (ScopedCompositeWorkflowContext, Result<(), E>)>,
    {
        let id = self.id;
        CURRENT_COMPOSITE_WORKFLOW_ID.scope(id, async {
            let (ctx, result) = f(self).await;
            clear_all_context(id);
            (ctx, result)
        }).await
    }

    pub fn store_return<T: 'static + Send>(&self, name: &'static str, value: T) {
        let mut guard = self.returns.lock().unwrap();
        guard.insert(name.to_string(), Box::new(value));
    }

    pub fn extract_return<T: 'static + Send>(&self, name: &str) -> Option<T> {
        let mut guard = self.returns.lock().unwrap();
        guard.remove(name)
            .and_then(|b| b.downcast::<T>().ok())
            .map(|b| *b)
    }
}

impl Default for ScopedCompositeWorkflowContext {
    fn default() -> Self {
        Self::new()
    }
}
