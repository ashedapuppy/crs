use ::safer_ffi::prelude::*;

use regex::Regex;

#[ffi_export]
fn rust_free_string (string: char_p::Box) {
    drop(string)
}

#[ffi_export]
fn len_str<'x>(s: char_p::Ref<'x>) -> &'x usize {
    &s.to_str().len()
}

// #[ffi_export]
// fn sep_str(string: char_p::Ref<'_>, separators: char_p::Ref<'_>) -> char_p::Box {
//     let s_safe = string.to_str();
//     let separators_safe = separators.to_str();

//     let mut str_vec = Vec::new();
//     let separator_re = Regex::new(format!("([{}]+)", separators_safe).as_str()).unwrap();

//     let split_s: Vec<&str> = separator_re.split(s_safe).into_iter().collect();

//     for &i in split_s.iter() {
//         str_vec.push(i.to_owned());
//     }
//     str_vec.try_into().unwrap()
// }

#[ffi_export]
fn cmp_str(a: char_p::Ref<'_>, b: char_p::Ref<'_>) -> i32 {
    let a_safe = a.to_str();
    let b_safe = b.to_str();

    (a_safe == b_safe) as i32
}

#[ffi_export]
fn dup_str(s: *const c_char) -> *mut c_char {
    let s_safe = from_cstr(s);
    let out = CString::new(s_safe);
    safe_unwrap(out).into_raw()
}

#[no_mangle]
pub extern "C" fn concat_str(a: *const c_char, b: *const c_char) -> *mut c_char {
    let a_safe = from_cstr(a);
    let b_safe = from_cstr(b);

    let out = CString::new([a_safe, b_safe].join(""));
    safe_unwrap(out).into_raw()
}