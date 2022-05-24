use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

use regex::Regex;

use crate::safe_unwrap;
use crate::CRSERR;

pub(crate) fn from_cstr(a: *const c_char) -> &'static str {
    safe_unwrap(
        unsafe {
            if a.is_null() {
                CRSERR = 1;
                return "";
            };
            CStr::from_ptr(a)
        }
        .to_str(),
    )
}

#[no_mangle]
pub extern "C" fn len_str(s: *const c_char) -> c_int {
    let s_safe = from_cstr(s);
    s_safe.chars().count() as c_int
}

#[no_mangle]
pub extern "C" fn sep_str(s: *const c_char, separators: *const c_char) -> *mut *mut c_char {
    let s_safe = from_cstr(s);
    let separators_safe = from_cstr(separators);

    let mut str_vec = Vec::new();
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        str_vec.push(safe_unwrap(CString::new(i)));
    }
    let mut out = str_vec
        .into_iter()
        .map(|s| s.into_raw())
        .collect::<Vec<_>>();
    out.push(std::ptr::null_mut());
    out.shrink_to_fit();
    assert_eq!(out.len(), out.capacity());

    let ptr = out.as_mut_ptr();
    std::mem::forget(out);
    ptr
}

#[no_mangle]
pub extern "C" fn eq_str(a: *const c_char, b: *const c_char) -> c_int {
    let a_safe = from_cstr(a);
    let b_safe = from_cstr(b);

    (a_safe == b_safe) as c_int
}

#[no_mangle]
pub extern "C" fn dup_str(s: *const c_char) -> *mut c_char {
    let s_safe = from_cstr(s);
    let out = CString::new(s_safe);
    safe_unwrap(out).into_raw()
}

#[no_mangle]
pub extern "C" fn concat_str(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a_safe = from_cstr(a);
    let b_safe = from_cstr(b);

    let out = CString::new([a_safe, b_safe].join(""));
    safe_unwrap(out).into_raw()
}

#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    unsafe {
        assert!(!s.is_null());
        CString::from_raw(s)
    };
}
