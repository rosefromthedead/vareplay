use std::ffi::c_void;

use libspa_sys::spa_pod;

extern "C" {
    pub fn build_video_format() -> *const spa_pod;
    pub fn on_param_changed(id: u32, param: *const spa_pod, out: *mut *const spa_pod);
    pub fn free(v: *const c_void);
    pub fn init_va() -> i32;
}
