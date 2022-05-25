use ::safer_ffi::prelude::*;

mod int;
mod str;

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("crs.h")?
        .generate()
}

/// `#[ffi_export]` is a Rust attribute that tells the Rust compiler to export the function to the C ABI
#[ffi_export]
fn hello_from_rust() {
    println!("Hello from Rust!");
}