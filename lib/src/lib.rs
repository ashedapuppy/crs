mod int;
mod math;
mod str;
mod vec;

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder().to_file("crs.h")?.generate()
}
