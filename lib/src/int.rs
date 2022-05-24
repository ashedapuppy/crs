use std::os::raw::c_char;
use std::os::raw::c_int;

use ::safer_ffi::prelude::*;

use regex::Regex;
use lazy_static::lazy_static;

use crate::CRSERR;

#[no_mangle]
pub extern "C" fn int_from_str(s: *const c_char) -> c_int {
    let s_safe = crate::str::from_cstr(s);
    // Creating a regular expression to find matches starting with a sign and
    // followed by one or more digits.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    if let Some(cap) = RE.captures(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<c_int>() {
                return number;
            }
        }
    }
    unsafe {
        CRSERR = -1;
    }
    return 0;
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IntArray {
    pub data: *mut c_int,
    pub len: usize,
}

#[ffi_export]
pub extern "C" fn ints_from_str(s: *const c_char) -> IntArray {
    let s_safe = crate::str::from_cstr(s);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    let mut int_vec: Vec<c_int> = Vec::new();
    for cap in RE.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<c_int>() {
                int_vec.push(number);
            }
        }
    }
    int_vec.push(0);
    let mut buf = int_vec.into_boxed_slice();
    let len = buf.len();
    let data =  buf.as_mut_ptr();
    IntArray { data, len }
}


#[no_mangle]
pub extern "C" fn ints_from_str_sep(s: *const c_char, separators: *const c_char) -> IntArray {
    let s_safe = crate::str::from_cstr(s);
    let separators_safe = crate::str::from_cstr(separators);

    let mut int_vec: Vec<c_int> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        if let Some(cap) = RE.captures(i) {
            if let Some(mat) = cap.get(0) {
                if let Ok(number) = mat.as_str().parse::<c_int>() {
                    int_vec.push(number);
                }
            }
        }
    }
    int_vec.push(0);
    let out = IntArray {
        data: int_vec.as_mut_ptr(), 
        len: int_vec.len(),
    };
    std::mem::forget(int_vec);
    std::mem::forget(&out);
    out
}
