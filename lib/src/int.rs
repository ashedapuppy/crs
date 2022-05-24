use std::os::raw::c_char;
use std::os::raw::c_int;

use regex::Regex;

use crate::CRSERR;

#[no_mangle]
pub extern "C" fn int_from_str(s: *const c_char) -> c_int {
    let s_safe = crate::str::from_cstr(s);
    // Creating a regular expression to find matches starting with a sign and
    // followed by one or more digits.
    let re = Regex::new(r"\+|-*\d+").unwrap();
    if let Some(cap) = re.captures(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<c_int>() {
                return number;
            }
        }
    }
    unsafe { CRSERR = -1; }
    return 0;
}

#[no_mangle]
pub extern "C" fn ints_from_str(s: *const c_char) -> *mut c_int {
    let s_safe = crate::str::from_cstr(s);
    let re = Regex::new(r"\+|-*\d+").unwrap();
    let mut int_vec: Vec<c_int> = Vec::new();
    for cap in re.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<c_int>() {
                int_vec.push(number);
            }
        }
    }
    int_vec.push(0);
    let ptr = int_vec.as_mut_ptr();
    std::mem::forget(int_vec);
    return ptr;
}

#[no_mangle]
pub extern "C" fn ints_from_str_sep(s: *const c_char, separators: *const c_char) -> *mut c_int {
    let s_safe = crate::str::from_cstr(s);
    let separators_safe = crate::str::from_cstr(separators);

    let mut int_vec: Vec<c_int> = Vec::new();
    let re = Regex::new(r"\+|-*\d+").unwrap();
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        if let Some(cap) = re.captures(i) {
            if let Some(mat) = cap.get(0) {
                if let Ok(number) = mat.as_str().parse::<c_int>() {
                    int_vec.push(number);
                }
            }
        }
    }
    int_vec.push(0);
    let ptr = int_vec.as_mut_ptr();
    std::mem::forget(int_vec);
    return ptr;
}

#[no_mangle]
pub extern "C" fn free_int_arr(s: *mut c_int) {
    unsafe {
        assert!(!s.is_null());
        Box::from_raw(s)
    };
}
