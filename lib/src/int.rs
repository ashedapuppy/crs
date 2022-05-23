use std::os::raw::c_char;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn int_from_str(s: *const c_char) -> c_int {
    let s_safe = crate::str::from_cstr(s);
    //todo: implement parsing an int from a string manually
    let s_int: c_int = s_safe.parse().unwrap_or_default();
    s_int
}
