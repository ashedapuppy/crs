#![allow(improper_ctypes_definitions)]
#![allow(unused_imports)]

use ::safer_ffi::prelude::*;

use regex::Regex;
use safer_ffi::{char_p::char_p_boxed, slice::slice_boxed, slice::slice_ref};

/// `rust_free_string` is a function that takes a `char_p_boxed` and drops it
///
/// Arguments:
///
/// * `string`: char_p_boxed - This is the string that we want to free.
#[ffi_export]
fn free_str(string: char_p_boxed) {
    drop(string)
}

/// `rust_free_string_array` is a function that takes a `slice::Box<char_p_boxed>` and drops it
///
/// Arguments:
///
/// * `arr`: slice::Box<char_p_boxed>
#[ffi_export]
fn free_str_arr(arr: slice_boxed<char_p_boxed>) {
    drop(arr)
}

#[ffi_export]
fn new_str_len(len: i32) -> char_p_boxed {
    char_p::new(String::with_capacity(len as usize))
}

#[ffi_export]
fn new_str_empty() -> char_p_boxed {
    char_p::new(String::new())
}

#[ffi_export]
fn uppr_str(s: char_p::Ref<'_>) -> char_p_boxed {
    char_p::new(s.to_str().to_uppercase())
}

#[ffi_export]
fn lwr_str(s: char_p::Ref<'_>) -> char_p_boxed {
    char_p::new(s.to_str().to_lowercase())
}

#[ffi_export]
fn pop_str(s: char_p::Ref<'_>, len: i32) -> char_p_boxed {
    let mut new = s.to_string();
    for _i in 0..len {
        new.pop();
    }
    char_p::new(new)
}

#[ffi_export]
fn push_str(s: char_p::Ref<'_>, c: u8) -> char_p_boxed {
    let mut new = s.to_string();
    new.push(c as char);
    char_p::new(new)
}

/// `len_str` takes a `char_p` (a pointer to a C-style string) and returns the length of the string
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_>
///
/// Returns:
///
/// A pointer to a string.
#[ffi_export]
fn len_str(s: char_p::Ref<'_>) -> usize {
    s.to_str().len()
}

macro_rules! unwrap_or_none {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return None,
        }
    };
}
pub(crate) use unwrap_or_none;

/// `sep_str` takes a string and a string of separators, and returns a slice of strings,
/// each of which is a substring of the original string
/// if no string can be found with the separators, len == 1
///
/// Arguments:
///
/// * `string`: char_p::Ref<'_>
/// * `separators`: A string of characters to split on.
///
/// Returns:
///
/// A slice of char_p_boxed
#[ffi_export]
fn split_str(s: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> slice_boxed<char_p_boxed> {
    let s_safe = s.to_str();
    let separators_safe = separators.to_str();
    let out: Box<[char_p_boxed]> = Box::new([char_p::new(String::new())]);

    let mut str_vec: Vec<&str> = Vec::new();
    if separators_safe.is_empty() {
        return out.try_into().unwrap();
    }
    let separator_re =
        Regex::new(format!("([{}]+)", separators_safe).as_str()).expect("could not build regex");

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        str_vec.push(i);
    }
    let out: Box<[char_p_boxed]> = str_vec.iter().map(|s| char_p::new(s.to_owned())).collect();
    out.try_into().unwrap()
}

#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> i32 {
    a.to_str().len() as i32 - b.to_str().len() as i32
}

/// `new_str` takes a `char*` and returns a `char*` that is a duplicate of the input
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_>
#[ffi_export]
fn new_str(s: char_p::Ref<'_>) -> char_p_boxed {
    s.to_string().try_into().unwrap()
}

/// `concat_str` takes two strings, concatenates them, and returns the result
///
/// Arguments:
///
/// * `a`: char_p::Ref<'_>
/// * `b`: char_p::Ref<'_>
#[ffi_export]
fn concat_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> char_p_boxed {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    let out = [a_safe, b_safe].join("");
    out.try_into().unwrap()
}
