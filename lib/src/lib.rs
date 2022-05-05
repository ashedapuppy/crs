use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod var;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

/// use this function as an example of how to create new ones
#[no_mangle]
pub extern "C" fn concat_str(a: *const c_char, b: *const c_char) -> *const c_char{
    let out;
    unsafe {
        let a_safe = CStr::from_ptr(a).to_str().unwrap();
        let b_safe = CStr::from_ptr(b).to_str().unwrap();
        out = CString::new(format!("{}{}", a_safe, b_safe))
            .unwrap();
    }
    var::str::persist(out.as_ptr())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
