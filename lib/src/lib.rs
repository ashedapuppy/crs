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

#[ffi_export]
fn hello_from_rust() {
    println!("Hello from Rust!");
}