#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoreRuntimeApi {
    pub get: extern "C" fn(key: *const std::os::raw::c_char) -> *mut std::ffi::c_void,
    pub set: extern "C" fn(key: *const std::os::raw::c_char, value: *mut std::ffi::c_void),
}