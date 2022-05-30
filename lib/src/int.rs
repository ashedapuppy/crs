#![allow(improper_ctypes_definitions)]

use ::safer_ffi::prelude::*;
use safer_ffi::char_p::char_p_boxed;
use safer_ffi::slice::slice_boxed;

use regex::Regex;

/// It takes a string and
/// returns an integer
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_> - A reference to a C string.
///
/// Returns:
///
/// integer from s
#[ffi_export]
fn int_from_str(s: char_p::Ref<'_>) -> i32 {
    let s_safe = s.to_str();
    // Creating a regular expression to find matches starting with a sign and
    // followed by one or more digits.
    let re: Regex = Regex::new(r"\+|-*\d+").expect("could not build regex");
    if let Some(cap) = re.captures(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<i32>() {
                return number;
            }
        }
    }
    0
}

#[ffi_export]
fn double_from_str(s: char_p::Ref<'_>) -> f64 {
    let s_safe = s.to_str();
    // Creating a regular expression to find matches starting with a sign and
    // followed by one or more digits.
    let re: Regex =
        Regex::new(r"[+-]?[0-9]+[.]?[0-9]*([e][+-]?[0-9]+)?").expect("could not build regex");
    if let Some(cap) = re.captures(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse::<f64>() {
                return number;
            }
        }
    }
    f64::NAN
}

#[ffi_export]
fn double_to_str(n: f64) -> char_p_boxed {
    char_p::new(format!("{}", n))
}

#[ffi_export]
fn int_to_str(n: i32) -> char_p_boxed {
    char_p::new(format!("{}", n))
}

#[ffi_export]
fn fmt_double(s: char_p::Ref<'_>) -> char_p_boxed {
    double_to_str(double_from_str(s))
}

#[ffi_export]
fn fmt_int(s: char_p::Ref<'_>) -> char_p_boxed {
    int_to_str(int_from_str(s))
}

/// It takes a string, extracts all the integers from it, and returns them as a slice
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_> - a pointer to a null-terminated string
///
/// Returns:
///
/// A slice of i32s.
#[ffi_export]
fn ints_from_str(s: char_p::Ref<'_>) -> slice_boxed<i32> {
    let s_safe = s.to_str();
    let mut arr: Box<[i32]> = Box::new([]);
    if s_safe.is_empty() {
        return arr.into();
    }
    let re: Regex = Regex::new(r"\+|-*\d+").expect("could not build regex");
    let mut int_vec: Vec<i32> = Vec::new();
    for cap in re.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse() {
                int_vec.push(number);
            }
        }
    }
    arr = int_vec.into();
    arr.into()
}

/// It takes a string, extracts all the numbers from it, and returns them as a slice of f64
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_> - a reference to a C string
///
/// Returns:
///
/// A slice of f64s
#[ffi_export]
fn doubles_from_str(s: char_p::Ref<'_>) -> slice_boxed<f64> {
    let s_safe = s.to_str();
    let mut arr: Box<[f64]> = Box::new([]);
    if s_safe.is_empty() {
        return arr.into();
    }
    let re: Regex =
        Regex::new(r"[+-]?[0-9]+[.]?[0-9]*([e][+-]?[0-9]+)?").expect("could not build regex");
    let mut double_vec: Vec<f64> = Vec::new();
    for cap in re.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse() {
                double_vec.push(number);
            }
        }
    }
    arr = double_vec.into();
    arr.into()
}

/// `rust_free_int_array` is a function that takes a slice of i32s and drops it
///
/// Arguments:
///
/// * `arr`: safer_ffi::slice::Box<i32>
#[ffi_export]
fn free_int_arr(arr: slice_boxed<i32>) {
    drop(arr);
}

/// `free_double_arr` takes a `slice_boxed<f64>` and drops it
///
/// Arguments:
///
/// * `arr`: slice_boxed<f64>
#[ffi_export]
fn free_double_arr(arr: slice_boxed<f64>) {
    drop(arr);
}
