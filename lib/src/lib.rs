mod int;
mod str;
mod vec;

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder().to_file("crs.h")?.generate()
}