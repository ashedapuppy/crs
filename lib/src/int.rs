#![allow(improper_ctypes_definitions)]

use ::safer_ffi::prelude::*;
use safer_ffi::slice::slice_boxed;

use lazy_static::lazy_static;
use regex::Regex;

use crate::str::unwrap_or_none;

/// `rust_free_int_array` is a function that takes a slice of i32s and drops it
///
/// Arguments:
///
/// * `arr`: safer_ffi::slice::Box<i32>
#[ffi_export]
fn free_int_arr(arr: safer_ffi::slice::Box<i32>) {
    drop(arr);
}

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
                return number;
            }
        }
    }
    0
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
fn ints_from_str(s: char_p::Ref<'_>) -> Option<slice_boxed<i32>> {
    let s_safe = s.to_str();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").expect("could not build regex");
    }
    let mut int_vec: Vec<i32> = Vec::new();
    for cap in RE.captures_iter(s_safe) {
        if let Some(mat) = cap.get(0) {
            if let Ok(number) = mat.as_str().parse() {
                int_vec.push(number);
            }
        }
    }
    let arr: Box<[i32]> = unwrap_or_none!(int_vec.try_into());
    Some(unwrap_or_none!(arr.try_into()))
}

/// It takes a string and a string of separators, splits the string on the separators, and returns a
/// slice of integers
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_> - a pointer to a string
/// * `separators`: a string of characters that are used to separate the numbers in the string.
///
/// Returns:
///
/// A slice of i32s.
#[ffi_export]
fn ints_from_str_sep(s: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> Option<slice_boxed<i32>> {
    let s_safe = s.to_str();
    let separators_safe = separators.to_str();

    let mut int_vec: Vec<i32> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|-*\d+").expect("could not build regex");
    }
    if separators_safe.is_empty() {
        return None;
    }
    let separator_re =
        Regex::new(format!("([{}]+)", separators_safe).as_str()).expect("could not build regex");

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
    let arr: Box<[i32]> = unwrap_or_none!(int_vec.try_into());
    Some(unwrap_or_none!(arr.try_into()))
}
