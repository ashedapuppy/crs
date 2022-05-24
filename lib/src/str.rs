use ::safer_ffi::prelude::*;

#[allow(unused_imports)]
use regex::Regex;

#[ffi_export]
fn rust_free_string (string: char_p::Box) {
    drop(string)
}

#[ffi_export]
fn rust_free_string_array (arr: safer_ffi::slice::Box<char_p::Box>) {
    drop(arr)
}

#[ffi_export]
fn len_str(s: char_p::Ref<'_>) -> usize {
    s.to_str().len()
}

#[ffi_export]
fn sep_str(string: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> safer_ffi::slice::Box<char_p::Box> {
    let s_safe = string.to_str();
    let separators_safe = separators.to_str();

    let mut str_vec: Vec<char_p::Box> = Vec::new();
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
        str_vec.push(i.to_owned().try_into().unwrap());
    };
    let out: Box<[char_p::Box]> = str_vec.try_into().unwrap();
    out.try_into().unwrap()
}

#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> i32 {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    (a_safe == b_safe) as i32
}

#[ffi_export]
fn dup_str(s: char_p::Ref<'_>) -> char_p::Box {
    let s_safe = s.to_string();
    s_safe.try_into().unwrap()
}

#[ffi_export]
fn concat_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> char_p::Box {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    let out = [a_safe, b_safe].join("");
    out.try_into().unwrap()
}