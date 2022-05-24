use ::safer_ffi::prelude::*;

use regex::Regex;
use lazy_static::lazy_static;

#[ffi_export]
fn rust_free_int_array(arr: safer_ffi::slice::Box<i32>) {
    drop(arr);
}

#[ffi_export]
fn int_from_str<'xs>(s: char_p::Ref<'_>) -> i32 {
    let s_safe = s.to_str();
    // Creating a regular expression to find matches starting with a sign and
    // followed by one or more digits.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    if let Some(cap) = RE.captures(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<i32>() {
                return number
            }
        }
    }
    0
}

#[ffi_export]
fn ints_from_str(s: char_p::Ref<'_>) -> safer_ffi::slice::Box<i32> {
    let s_safe = s.to_str();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    let mut int_vec: Vec<i32> = Vec::new();
    for cap in RE.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse() {
                int_vec.push(number);
            }
        }
    }
    let arr: Box<[i32]> = int_vec.try_into().unwrap();
    arr.try_into().unwrap()
}


#[ffi_export]
fn ints_from_str_sep(s: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> safer_ffi::slice::Box<i32> {
    let s_safe = s.to_str();
    let separators_safe = separators.to_str();

    let mut int_vec: Vec<i32> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").unwrap();
    }
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        if let Some(cap) = RE.captures(i) {
            if let Some(mat) = cap.get(0) {
                if let Ok(number) = mat.as_str().parse() {
                    int_vec.push(number);
                }
            }
        }
    }
    let arr: Box<[i32]> = int_vec.try_into().unwrap();
    arr.try_into().unwrap()
}
