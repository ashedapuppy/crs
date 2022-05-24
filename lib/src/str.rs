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

<<<<<<< HEAD
#[no_mangle]
pub extern "C" fn sep_str(s: *const c_char, separators: *const c_char) -> *mut *mut c_char {
    let s_safe = from_cstr(s);
    let separators_safe = from_cstr(separators);
=======
#[ffi_export]
fn sep_str(string: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> safer_ffi::slice::Box<char_p::Box> {
    let s_safe = string.to_str();
    let separators_safe = separators.to_str();
>>>>>>> safer_ffi

    let mut str_vec: Vec<char_p::Box> = Vec::new();
    let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

    let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

    for &i in split_s.iter() {
<<<<<<< HEAD
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
=======
        str_vec.push(i.to_owned().try_into().unwrap());
    };
    let out: Box<[char_p::Box]> = str_vec.try_into().unwrap();
    out.try_into().unwrap()
}

#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> i32 {
    let a_safe = a.to_str();
    let b_safe = b.to_str();
>>>>>>> safer_ffi

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