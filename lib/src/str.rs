#![allow(improper_ctypes_definitions)]

use ::safer_ffi::prelude::*;

#[allow(unused_imports)]
use regex::Regex;
use safer_ffi::slice;

/// `rust_free_string` is a function that takes a `char_p::Box` and drops it
/// 
/// Arguments:
/// 
/// * `string`: char_p::Box - This is the string that we want to free.
#[ffi_export]
fn rust_free_string (string: char_p::Box) {
    drop(string)
}

/// `rust_free_string_array` is a function that takes a `slice::Box<char_p::Box>` and drops it
/// 
/// Arguments:
/// 
/// * `arr`: slice::Box<char_p::Box>
#[ffi_export]
fn rust_free_string_array (arr: slice::Box<char_p::Box>) {
    drop(arr)
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
    }
}
pub(crate) use unwrap_or_none;

/// `sep_str` takes a string and a string of separators, and returns a slice of strings, 
/// each of which is a substring of the original string
/// 
/// Arguments:
/// 
/// * `string`: char_p::Ref<'_>
/// * `separators`: A string of characters to split on.
/// 
/// Returns:
/// 
/// A slice of char_p::Box
#[ffi_export]
fn sep_str(string: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> 
    Option<slice::Box<char_p::Box>> 
{
    let s_safe = string.to_str();
    let separators_safe = separators.to_str();

    let mut str_vec: Vec<char_p::Box> = Vec::new();
    let separator_re = 
        Regex::new(format!("([{}]+)", separators_safe).as_str()).expect("could not build regex");

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        str_vec.push(unwrap_or_none!(i.to_owned().try_into()));
    };
    let out: Box<[char_p::Box]> = unwrap_or_none!(str_vec.try_into());
    Some(unwrap_or_none!(out.try_into()))
}

/// `cmp_str` takes two `char*` pointers, converts them to Rust strings, compares them, and returns a
/// boolean as an `i32`
/// 
/// Arguments:
/// 
/// * `a`: char_p::Ref<'_> - This is a reference to a pointer to a C string.
/// * `b`: char_p::Ref<'_>
/// 
/// Returns:
/// 
/// A boolean value is being returned.
#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> i32 {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    (a_safe == b_safe) as i32
}


/// `dup_str` takes a `char*` and returns a `char*` that is a duplicate of the input
/// 
/// Arguments:
/// 
/// * `s`: char_p::Ref<'_>
#[ffi_export]
fn dup_str(s: char_p::Ref<'_>) -> char_p::Box {
    let s_safe = s.to_string();
    s_safe.try_into().unwrap()
}

/// `concat_str` takes two strings, concatenates them, and returns the result
/// 
/// Arguments:
/// 
/// * `a`: char_p::Ref<'_>
/// * `b`: char_p::Ref<'_>
#[ffi_export]
fn concat_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> char_p::Box {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    let out = [a_safe, b_safe].join("");
    out.try_into().unwrap()
}