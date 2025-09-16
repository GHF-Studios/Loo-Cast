use core_api::bevy::ecs::entity::Entity;
use core_api::config::structs::Config;
use core_api::crossbeam::queue::SegQueue;
use core_api::logging::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};
use core_api::once_cell::sync::Lazy;
use core_api::time::types::PendingSleep;
use core_api::tokio::runtime::Runtime;
use core_api::workflow::types::CompositeWorkflowRuntime;
use core_runtime_api::CoreRuntimeApi;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::ptr::NonNull;
use std::sync::{atomic::AtomicU64, Mutex, RwLock};
use std::time::Instant;

/// A registry pointer. Guaranteed by engine to be safe for cross-thread usage.
#[derive(Copy, Clone)]
pub struct RegistryPtr(NonNull<c_void>);

unsafe impl Send for RegistryPtr {}
unsafe impl Sync for RegistryPtr {}

static REGISTRY: Lazy<RwLock<HashMap<&'static str, RegistryPtr>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn register<T: 'static>(key: &'static str, value: &'static Lazy<T>) {
    println!("Registering static {key}");
    let inner: &T = Lazy::force(value); // this runs init if not done yet
    let mut reg = REGISTRY.write().unwrap();
    let ptr = NonNull::new(inner as *const T as *mut c_void).unwrap();
    reg.insert(key, RegistryPtr(ptr));
}

extern "C" fn get(key: *const c_char) -> *mut c_void {
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
    let reg = REGISTRY.read().unwrap();
    reg.get(key).map(|rp| rp.0.as_ptr()).unwrap_or(std::ptr::null_mut())
}

extern "C" fn set(key: *const c_char, value: *mut c_void) {
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
    let mut reg = REGISTRY.write().unwrap();
    let ptr = NonNull::new(value).unwrap();
    reg.insert(Box::leak(key.to_string().into_boxed_str()), RegistryPtr(ptr));
}

pub fn get_api() -> CoreRuntimeApi {
    CoreRuntimeApi { get, set }
}

pub fn init_statics() {
    static CONFIG: Lazy<Config> = Lazy::new(core_api::config::statics::init_config);

    static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(core_api::core::statics::init_tokio_runtime);
    static START_TIME: Lazy<Instant> = Lazy::new(core_api::core::statics::init_start_time);

    static ENTITY_RESERVATION_BUFFER: Lazy<Mutex<Vec<Entity>>> = Lazy::new(core_api::entity::statics::init_entity_reservation_buffer);

    static LOG_ID_COUNTER: Lazy<AtomicU64> = Lazy::new(core_api::logging::statics::init_log_id_counter);
    static SPAN_EVENT_BUFFER: Lazy<SegQueue<SpanPath>> = Lazy::new(core_api::logging::statics::init_span_event_buffer);
    static LOG_EVENT_BUFFER: Lazy<SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)>> = Lazy::new(core_api::logging::statics::init_log_event_buffer);

    static ELAPSED_VIRTUAL_NANOS: Lazy<AtomicU64> = Lazy::new(core_api::time::statics::init_elapsed_virtual_nanos);
    static PENDING_VIRTUAL_SLEEPS: Lazy<Mutex<Vec<PendingSleep>>> = Lazy::new(core_api::time::statics::init_pending_virtual_sleeps);

    static COMPOSITE_WORKFLOW_RUNTIME: Lazy<Mutex<CompositeWorkflowRuntime>> = Lazy::new(core_api::workflow::statics::init_composite_workflow_runtime);
    static WORKFLOW_TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(core_api::workflow::statics::init_workflow_tokio_runtime);

    register("config", &CONFIG);
    register("tokio_runtime", &TOKIO_RUNTIME);
    register("start_time", &START_TIME);

    register("entity_reservation_buffer", &ENTITY_RESERVATION_BUFFER);

    register("log_id_counter", &LOG_ID_COUNTER);
    register("span_event_buffer", &SPAN_EVENT_BUFFER);
    register("log_event_buffer", &LOG_EVENT_BUFFER);

    register("elapsed_virtual_nanos", &ELAPSED_VIRTUAL_NANOS);
    register("pending_virtual_sleeps", &PENDING_VIRTUAL_SLEEPS);

    register("workflow_tokio_runtime", &WORKFLOW_TOKIO_RUNTIME);
    register("composite_workflow_runtime", &COMPOSITE_WORKFLOW_RUNTIME);
}

