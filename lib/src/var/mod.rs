use std::mem::transmute;
use std::sync::Once;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

pub static STRSTART: Once = Once::new();
pub static mut STRDATA:*const CString = 0 as *const CString;

/// make string usable in C. If this is not used, 
/// the string gets out of scope, resulting in UB
#[no_mangle]
pub(crate) extern "C" fn strpersist(a: *const c_char) -> *const c_char{
    STRSTART.call_once(|| {
        unsafe {
            let a_safe = CStr::from_ptr(a).to_str().unwrap();
            let boxed = Box::new(CString::new(a_safe.as_bytes()).unwrap());
            STRDATA = transmute(boxed);
        }
    });
    unsafe {
        return (&*STRDATA).as_ptr()
    }
}
