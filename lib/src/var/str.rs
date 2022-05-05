use std::mem::transmute;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;
use std::sync::Once;

pub static START: Once = Once::new();
pub static mut DATA:*const CString = 0 as *const CString;

/// make string usable in C. If this is not used, 
/// the string gets out of scope, resulting in UB
#[no_mangle]
pub(crate) extern "C" fn persist(a: *const c_char) -> *const c_char{
    START.call_once(|| {
        unsafe {
            let a_safe = CStr::from_ptr(a).to_str().unwrap();
            let boxed = Box::new(CString::new(a_safe.as_bytes()).unwrap());
            DATA = transmute(boxed);
        }
    });
    unsafe {
        return (&*DATA).as_ptr()
    }
}
