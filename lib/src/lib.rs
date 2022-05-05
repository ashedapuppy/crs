use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

mod var;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

/// use this function as an example of how to create new ones
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
