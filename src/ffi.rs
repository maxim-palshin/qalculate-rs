use std::ffi::c_void;
use std::os::raw::c_char;

#[repr(C)]
pub struct QalculateState_c {
    pub calc: *mut c_void,
}

unsafe extern "C" {
    pub fn init() -> QalculateState_c;
    pub fn calculate(expression: *const c_char, state: *mut QalculateState_c) -> f64;
    pub fn destroy(state: *mut QalculateState_c) -> f64;
    pub fn calculate_string(expression: *const c_char, state: *mut QalculateState_c)
    -> *mut c_char;
    pub fn load_global_definitions(state: *mut QalculateState_c);
}
