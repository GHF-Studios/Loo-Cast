use core_runtime_api::CoreRuntimeApi;
use std::sync::OnceLock;

/// The one and only CoreRuntimeApi handle (set by core_runtime at startup).
static API: OnceLock<&'static CoreRuntimeApi> = OnceLock::new();

/// Called from engine to give core_api access to the runtime registry.
pub fn init_runtime_api(api: &'static CoreRuntimeApi) {
    API.set(api).ok();
}

/// Internal helper to get the API everywhere else.
fn api() -> &'static CoreRuntimeApi {
    API.get().expect("CoreRuntimeApi not initialized")
}

/// Fetch a raw pointer from the registry by key.
pub fn get_ptr(key: &str) -> *mut std::ffi::c_void {
    use std::ffi::CString;
    let cstr = CString::new(key).unwrap();
    (api().get)(cstr.as_ptr())
}

/// Store/replace a pointer in the registry by key.
pub fn set_ptr(key: &str, value: *mut std::ffi::c_void) {
    use std::ffi::CString;
    let cstr = CString::new(key).unwrap();
    (api().set)(cstr.as_ptr(), value);
}

/// Type-safe getter
pub fn get_ref<T>(key: &str) -> &'static T {
    let ptr = get_ptr(key);
    assert!(!ptr.is_null(), "Static {key} not found in registry");
    unsafe { &*(ptr as *const T) }
}
