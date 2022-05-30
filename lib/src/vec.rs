use ::safer_ffi::prelude::*;
use safer_ffi::char_p::*;

#[derive_ReprC]
#[repr(C)]
#[derive(Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[ffi_export]
fn free_vec(v: Vector) {
    drop(v)
}

#[ffi_export]
fn new_vec(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z}
}

#[ffi_export]
fn dup_vec(v: Vector) -> Vector {
    new_vec(v.x, v.y, v.z)
}

#[ffi_export]
fn fmt_vec(v: Vector) -> char_p_boxed {
    char_p::new(format!("({}, {}, {})", v.x, v.y, v.z))
}

#[ffi_export]
fn print_vec(v: Vector) {
    print!("({}, {}, {})", v.x, v.y, v.z);
}

#[ffi_export]
fn add_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    u.x += v.x;
    u.y += v.y;
    u.z += v.z;
    u
}

#[ffi_export]
fn sub_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    u.x -= v.x;
    u.y -= v.y;
    u.z -= v.z;
    u
}

#[ffi_export]
fn add_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    u.x += x;
    u.y += y;
    u.z += z;
    u
}

#[ffi_export]
fn sub_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    u.x -= x;
    u.y -= y;
    u.z -= z;
    u
}

#[ffi_export]
fn mul_vec<'x>(u: &'x mut Vector, multiplicand: f64) -> &'x mut Vector {
    u.x *= multiplicand;
    u.y *= multiplicand;
    u.z *= multiplicand;
    u
}

#[ffi_export]
fn dotprod_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    u.x *= v.x;
    u.y *= v.y;
    u.z *= v.z;
    u
}

#[ffi_export]
fn div_vec<'x>(u: &'x mut Vector, dividand: f64) -> &'x mut Vector {
    u.x /= dividand;
    u.y /= dividand;
    u.z /= dividand;
    u
}

#[ffi_export]
fn magnitude_vec(u: &'_ Vector) -> f64 {
    f64::sqrt(u.x * u.x + u.y * u.y + u.z * u.z)
}

#[ffi_export]
fn normalise_vec<'x>(u: &'x mut Vector) -> &'x mut Vector {
    let mag = magnitude_vec(u);
    u.x /= mag;
    u.y /= mag;
    u.z /= mag;
    u
}