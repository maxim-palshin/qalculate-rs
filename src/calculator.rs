use crate::ffi;
use std::ffi::{CStr, CString};
use libc;


pub struct Qalculate {
    state: Box<ffi::QalculateState_c>,
}

impl Qalculate {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let state = ffi::init();
            if state.calc.is_null() {
                return Err("Failed to initialize calculator".to_string());
            }
            Ok(Qalculate {
                state: Box::new(state),
            })
        }
    }

    pub fn calculate(&self, expression: &str) -> Result<f64, String> {
        let c_expr = match CString::new(expression) {
            Ok(s) => s,
            Err(e) => return Err(format!("Invalid expression: {}", e)),
        };

        unsafe {
            let state_ptr =
                &*self.state as *const ffi::QalculateState_c as *mut ffi::QalculateState_c;

            let result = ffi::calculate(c_expr.as_ptr(), state_ptr);
            if result.is_nan() {
                return Err("Calculation returned NaN".to_string());
            }

            Ok(result)
        }
    }

    pub fn calculate_string(&self, expression: &str) -> Result<String, String> {
        let c_expr = match CString::new(expression) {
            Ok(s) => s,
            Err(e) => return Err(format!("Invalid expression: {}", e)),
        };

        unsafe {
            let state_ptr =
                &*self.state as *const ffi::QalculateState_c as *mut ffi::QalculateState_c;
            let result_ptr = ffi::calculate_string(c_expr.as_ptr(), state_ptr);

            if result_ptr.is_null() {
                return Err("Calculation failed or returned null".to_string());
            }

            let c_str = CStr::from_ptr(result_ptr);
            let result = c_str.to_string_lossy().into_owned();

             libc::free(result_ptr as *mut _);

            Ok(result)
        }
    }
}

impl Drop for Qalculate {
    fn drop(&mut self) {
        unsafe {
            let state_ptr =
                &*self.state as *const ffi::QalculateState_c as *mut ffi::QalculateState_c;
            ffi::destroy(state_ptr);
        }
    }
}
