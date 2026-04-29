use crate::bevy::prelude::Reflect;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::utils::premium_box::AnySendSyncPremiumBox;

#[derive(Hash, Eq, PartialEq, Reflect)]
pub(super) struct ContextKey {
    type_id: TypeId,
    name: &'static str,
}

#[derive(Default, Reflect)]
pub struct CompositeWorkflowContext {
    map: HashMap<ContextKey, AnySendSyncPremiumBox>,
}

static CONTEXTS: Lazy<DashMap<Uuid, Arc<Mutex<CompositeWorkflowContext>>>> = Lazy::new(DashMap::new);

tokio::task_local! {
    pub static CURRENT_COMPOSITE_WORKFLOW_ID: Uuid;
}

pub fn set_context<T: 'static + Send + Sync>(name: &'static str, val: T) {
    let id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS
        .get(&id)
        .unwrap_or_else(|| unreachable!("Missing workflow context for `{}`", name))
        .clone();
    let mut ctx = ctx.lock().unwrap_or_else(|_| unreachable!("Workflow context mutex poisoned for `{}`", name));

    ctx.map.insert(
        ContextKey {
            type_id: TypeId::of::<T>(),
            name,
        },
        AnySendSyncPremiumBox::new(Some(val), std::any::type_name::<T>().to_string()),
    );
}

pub fn get_context<T: 'static + Send + Sync>(name: &'static str) -> T {
    let id = CURRENT_COMPOSITE_WORKFLOW_ID.with(|id| *id);
    let ctx = CONTEXTS
        .get(&id)
        .unwrap_or_else(|| unreachable!("Missing workflow context for `{}`", name))
        .clone();
    let mut ctx = ctx.lock().unwrap_or_else(|_| unreachable!("Workflow context mutex poisoned for `{}`", name));

    ctx.map
        .get_mut(&ContextKey {
            type_id: TypeId::of::<T>(),
            name,
        })
        .expect("Context value not found for `{}`")
        .inner_mut::<Option<T>>()
        .take()
        .unwrap_or_else(|| unreachable!("Context value was empty for `{}`", name))
}

pub fn clear_all_context(id: Uuid) {
    CONTEXTS.remove(&id);
}

#[derive(Reflect)]
pub struct ScopedCompositeWorkflowContext {
    pub id: Uuid,
    #[reflect(ignore)]
    pub returns: Arc<Mutex<HashMap<String, AnySendSyncPremiumBox>>>,
}
impl std::fmt::Debug for ScopedCompositeWorkflowContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
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
        CURRENT_COMPOSITE_WORKFLOW_ID.scope(id, async { f(self).await }).await
    }

    pub async fn run_fallible<F, Fut, E>(self, f: F) -> (ScopedCompositeWorkflowContext, Result<(), E>)
    where
        F: FnOnce(Self) -> Fut,
        Fut: std::future::Future<Output = (ScopedCompositeWorkflowContext, Result<(), E>)>,
    {
        let id = self.id;
        CURRENT_COMPOSITE_WORKFLOW_ID.scope(id, async { f(self).await }).await
    }

    pub fn store_return<T: 'static + Send + Sync>(&self, name: &'static str, value: T) {
        let mut guard = self.returns.lock().unwrap();
        guard.insert(name.to_string(), AnySendSyncPremiumBox::new(value, std::any::type_name::<T>().to_string()));
    }

    pub fn extract_return<T: 'static + Send + Sync>(&self, name: &str) -> Option<T> {
        let mut guard = self.returns.lock().unwrap();
        guard.remove(name).map(|b| b.into_inner::<T>())
    }
}

impl Default for ScopedCompositeWorkflowContext {
    fn default() -> Self {
        Self::new()
    }
}
