use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

mod var;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

/// `concat_strs` takes two C strings, converts them to Rust strings, concatenates them, 
/// and returns a C string
/// 
/// Arguments:
/// 
/// * `a`: *const c_char
/// * `b`: *const c_char
/// 
/// Returns:
/// 
/// A persistent C string
#[no_mangle]
pub extern "C" fn concat_strs(a: *const c_char, b: *const c_char) -> *const c_char{
    let out;
    unsafe {
        let a_safe = CStr::from_ptr(a).to_str().unwrap();
        let b_safe = CStr::from_ptr(b).to_str().unwrap();
        out = CString::new(format!("{}{}", a_safe, b_safe))
            .unwrap();
    }
    var::strpersist(out.as_ptr())
}

/// `int_from_str` takes a pointer to a C string, converts it to a Rust string, 
/// parses it to an integer, and returns the integer
/// 
/// Arguments:
/// 
/// * `a`: *const c_char
/// 
/// Returns:
/// 
/// A c_int
#[no_mangle]
pub extern "C" fn int_from_str(a: *const c_char) -> c_int{
    let a_int: c_int;
    unsafe {
        let a_safe = CStr::from_ptr(a).to_str().unwrap();
        a_int = a_safe.parse().unwrap_or_default();
    }
    a_int
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_int_from_str() {
        let c_str1 = std::ffi::CString::new("123".as_bytes()).unwrap();
        assert_eq!(int_from_str(c_str1.as_ptr()), 123);

        let c_str2 = std::ffi::CString::new("abc".as_bytes()).unwrap();
        assert_eq!(int_from_str(c_str2.as_ptr()), 0);

        let c_str3 = std::ffi::CString::new("99999999999999999999999999999".as_bytes()).unwrap();
        assert_eq!(int_from_str(c_str3.as_ptr()), 0);
    }
}
