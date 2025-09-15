use core_runtime_api::CoreRuntimeApi;

use core_api::config::structs::Config;
use core_api::entity::Entity;
use core_api::logging::types::{LogEntry, LogId, ModulePath, PhysicalStoragePath, SpanPath};
use core_api::time::types::PendingSleep;
use core_api::workflow::types::CompositeWorkflowRuntime;

use core_api::crossbeam::queue::SegQueue;
use core_api::once_cell::sync::Lazy;
use core_api::std::collections::HashMap;
use core_api::std::sync::{atomic::AtomicU64, Mutex, RwLock};
use core_api::std::time::Instant;
use core_api::tokio::runtime::Runtime;

static REGISTRY: Lazy<RwLock<HashMap<&'static str, *mut std::ffi::c_void>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Register a value by key (usually called at init).
pub fn register<T: 'static>(key: &'static str, value: &'static T) {
    let mut reg = REGISTRY.write().unwrap();
    reg.insert(key, value as *const T as *mut std::ffi::c_void);
}

/// Get a pointer from the registry by key.
extern "C" fn get(key: *const std::os::raw::c_char) -> *mut std::ffi::c_void {
    let key = unsafe { std::ffi::CStr::from_ptr(key).to_str().unwrap() };
    let reg = REGISTRY.read().unwrap();
    reg.get(key).copied().unwrap_or(std::ptr::null_mut())
}

/// Update a key with a new pointer (for mutables / dynamic rebinds).
extern "C" fn set(key: *const std::os::raw::c_char, value: *mut std::ffi::c_void) {
    let key = unsafe { std::ffi::CStr::from_ptr(key).to_str().unwrap() };
    let mut reg = REGISTRY.write().unwrap();
    reg.insert(Box::leak(key.to_string().into_boxed_str()), value);
}

pub fn build_runtime_api() -> CoreRuntimeApi {
    CoreRuntimeApi { get, set }
}

pub fn init_statics() {
    static CONFIG: Config = Config::from_file("configs/config.toml").unwrap();
    static TOKIO_RUNTIME: Runtime = Runtime::new().unwrap();
    static START: Instant = Instant::now();

    static ENTITY_BUF: Mutex<Vec<Entity>> = Mutex::new(Vec::new());

    static LOG_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
    static SPAN_BUFFER: SegQueue<SpanPath> = SegQueue::new();
    static LOG_BUFFER: SegQueue<(LogId, LogEntry, SpanPath, ModulePath, PhysicalStoragePath)> = SegQueue::new();

    static ELAPSED: AtomicU64 = AtomicU64::new(0);
    static SLEEPS: Mutex<Vec<PendingSleep>> = Mutex::new(vec![]);

    static WF_RUNTIME: CompositeWorkflowRuntime = CompositeWorkflowRuntime::new();
    static WF_RT: Runtime = Runtime::new().unwrap();

    register("config", &CONFIG);
    register("tokio_runtime", &RUNTIME);
    register("start_time", &START);

    register("entity_reservation_buffer", &ENTITY_BUF);

    register("log_id_counter", &LOG_ID_COUNTER);
    register("span_event_buffer", &SPAN_BUFFER);
    register("log_event_buffer", &LOG_BUFFER);

    register("elapsed_virtual_nanos", &ELAPSED);
    register("pending_virtual_sleeps", &SLEEPS);

    register("workflow_tokio_runtime", &WF_RT);
    register("composite_workflow_runtime", &WF_RUNTIME);
}

