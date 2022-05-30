#![allow(improper_ctypes_definitions)]
#![allow(unused_imports)]

use ::safer_ffi::prelude::*;
use safer_ffi::{char_p::char_p_boxed, slice::slice_boxed, slice::slice_ref};
use regex::Regex;

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
fn split_str(s: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> 
    slice_boxed<char_p_boxed> 
{
    let s_safe = s.to_str();
    let separators_safe = separators.to_str();
    // empty string array to be filled and returned
    let mut out: Box<[char_p_boxed]> = Box::new([char_p::new(String::new())]);

    if separators_safe.is_empty() || s_safe.is_empty() {
        return out.into();
    }
    let separator_re =
        Regex::new(format!("[{}]+", separators_safe).as_str())
        .expect("could not build regex");

    // take all the separated tokens found with the regex and add them to a string vector
    let mut str_vec: Vec<&str> = Vec::new();
    for token in separator_re.split(s_safe).into_iter() {
        str_vec.push(token);
    }

    // convert the string vector to the correct return type
    out = str_vec.iter().map(|s| char_p::new(s.to_owned())).collect();
    out.into()
}

/// `cmp_str` takes two `char_p`s, converts them to Rust strings, and compares them character by
/// character
/// 
/// Arguments:
/// 
/// * `a`: char_p::Ref<'_> - This is a pointer to a C string.
/// * `b`: char_p::Ref<'_>
/// 
/// Returns:
/// 
/// The difference between the first two characters that are different.
#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> isize {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    for (i, c) in a_safe.chars().enumerate() {
        if let Some(c2) = b_safe.chars().nth(i) {
            if c != c2 {
                return c as isize - c2 as isize;
            }
        } else {
            return c as isize;
        }
    }
    0
}

/// `new_str` takes a `char*` and returns a `char*` that is a duplicate of the input
///
/// Arguments:
///
/// * `s`: char_p::Ref<'_>
#[ffi_export]
fn new_str(s: char_p::Ref<'_>) -> char_p_boxed {
    let s_safe = s.to_str();
    if s_safe.is_empty() {
        return char_p::new(String::new());
    }
    char_p::new(String::from(s_safe))
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
    if a_safe.is_empty() || b_safe.is_empty() {
        return char_p::new(String::new());
    }
    char_p::new([a_safe, b_safe].join(""))
}

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use super::*;
    use ::safer_ffi::prelude::*;
    use safer_ffi::{char_p::{char_p_boxed, char_p_ref}, slice::slice_boxed, slice::slice_ref};
    #[test]
    fn test_concat() {
        let a = CString::new("hello").unwrap();
        let b = CString::new("world").unwrap();
        let c = concat_str(
            char_p_ref::from(a.as_c_str()), 
            char_p_ref::from(b.as_c_str()));
        assert_eq!(c.to_str(), "helloworld");
    }

    #[test]
    fn test_new_string() {
        let a = CString::new("hello").unwrap();
        let b = new_str(char_p_ref::from(a.as_c_str()));
        assert_eq!(b.to_str(), "hello");
    }

    #[test]
    fn test_cmp_str() {
        let a = CString::new("helloo").unwrap();
        let b = CString::new("helloo").unwrap();
        let c = cmp_str(
            char_p_ref::from(a.as_c_str()), 
            char_p_ref::from(b.as_c_str()));
        assert_eq!(c, 0);
    }

    #[test]
    fn test_split_str() {
        let a = CString::new("hello,world !").unwrap();
        let b = CString::new(", ").unwrap();
        let c = split_str(
            char_p_ref::from(a.as_c_str()), 
            char_p_ref::from(b.as_c_str()));
        assert_eq!(c.len(), 3);
        assert_eq!(c[0].to_str(), "hello");
        assert_eq!(c[1].to_str(), "world");
        assert_eq!(c[2].to_str(), "!");
    }
    
    #[test]
    fn test_len_str() {
        let a = CString::new("hello").unwrap();
        let b = len_str(char_p_ref::from(a.as_c_str()));
        assert_eq!(b, 5);
    }

    #[test]
    fn test_push_str() {
        let a = CString::new("hello").unwrap();
        let b = push_str(
            char_p_ref::from(a.as_c_str()), 
            '!' as u8);
        assert_eq!(b.to_str(), "hello!");
    }

    #[test]
    fn test_pop_str() {
        let a = CString::new("hello!").unwrap();
        let b = pop_str(
            char_p_ref::from(a.as_c_str()),
            1);
        assert_eq!(b.to_str(), "hello");
    }

    #[test]
    fn test_lwr_str() {
        let a = CString::new("HELLO!").unwrap();
        let b = lwr_str(
            char_p_ref::from(a.as_c_str()));
        assert_eq!(b.to_str(), "hello!");
    }

    #[test]
    fn test_uppr_str() {
        let a = CString::new("hello!").unwrap();
        let b = uppr_str(
            char_p_ref::from(a.as_c_str()));
        assert_eq!(b.to_str(), "HELLO!");
    }

    #[test]
    fn test_new_str_empty() {
        let b = new_str_empty();
        assert_eq!(b.to_str(), "");
    }
}