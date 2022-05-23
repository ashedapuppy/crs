#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

mod str {
    use std::ffi::CString;
    use std::ffi::CStr;
    use std::os::raw::c_int;
    use std::os::raw::c_char;

    pub(crate) fn from_cstr(a: *const c_char) -> &'static str {
        unsafe {  
            assert!(!a.is_null());
            CStr::from_ptr(a)
        }.to_str().unwrap()
    }

    #[no_mangle]
    pub extern "C" fn len_str(s: *const c_char) -> c_int {
        let s_safe = from_cstr(s);
        s_safe.chars().count() as c_int
    }

    #[no_mangle]
    pub extern "C" fn cmp_str(a: *const c_char, b: *const c_char) -> c_int {
        let a_safe = from_cstr(a);
        let b_safe = from_cstr(b);

        (a_safe == b_safe) as c_int
    }

    #[no_mangle]
    pub extern "C" fn dup_str(s: *const c_char) -> *mut c_char {
        let s_safe = from_cstr(s);
        let out = CString::new(s_safe).unwrap();
        out.into_raw()
    }

    #[no_mangle]
    pub extern "C" fn concat_str(a: *const c_char, b: *const c_char) -> *mut c_char{
        let a_safe = from_cstr(a);
        let b_safe = from_cstr(b); 

        let out = {
            let mut out_array: Vec<u8> = Vec::new();

            for c in a_safe.chars() {
               out_array.push(c as u8)
            }
            for c in b_safe.chars() {
                out_array.push(c as u8)
            }

            CString::new(out_array)
                .unwrap()
        };
        out.into_raw()
    }

    #[no_mangle]
    pub extern "C" fn free_str(s: *mut c_char) {
        unsafe {
            assert!(!s.is_null());
            CString::from_raw(s)
        };
    }
}

mod int {
    use std::os::raw::c_int;
    use std::os::raw::c_char;

    #[no_mangle]
    pub extern "C" fn int_from_str(s: *const c_char) -> c_int{
        let s_safe = crate::str::from_cstr(s);
        //todo: implement parsing an int from a string manually
        let s_int: c_int = s_safe.parse().unwrap_or_default();
        s_int
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_int_from_str() {
        let c_str1 = std::ffi::CString::new("123".as_bytes()).unwrap();
        assert_eq!(int::int_from_str(c_str1.as_ptr()), 123);

        let c_str2 = std::ffi::CString::new("abc".as_bytes()).unwrap();
        assert_eq!(int::int_from_str(c_str2.as_ptr()), 0);

        let c_str3 = std::ffi::CString::new("99999999999999999999999999999".as_bytes()).unwrap();
        assert_eq!(int::int_from_str(c_str3.as_ptr()), 0);
    }

    // fn test_concat_string() {
    //     let c_str1 = std::ffi::CString::new("abc".as_bytes()).unwrap();
    //     let c_str2 = std::ffi::CString::new("123".as_bytes()).unwrap();
    //     let out = std::ffi::CString::new("abc123".as_bytes()).unwrap();
    //     assert_eq!(concat_strs(c_str1.as_ptr(), c_str2.as_ptr())), out);
    // }
}
