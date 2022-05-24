use std::os::raw::c_int;

mod int;
mod str;

/// This is a global variable that is used to store the error code of the last function called.
/// -1: result not found
/// 0: no error
/// 1: Null pointer passed to rust function
/// 2: unwrap failed (called unwrap on Err value)
#[no_mangle]
#[used]
pub static mut CRSERR: c_int = 0;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

pub(crate) fn safe_unwrap<T, E>(r: Result<T, E>) -> T
where
    T: Default,
    E: std::fmt::Debug,
{
    match r {
        Ok(t) => t,
        Err(_) => {
            unsafe { CRSERR = 2 }
            T::default()
        }
    }
}
