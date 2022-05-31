use ::safer_ffi::prelude::*;
use safer_ffi::char_p::*;
use std::ffi::CString;

use super::Vector;
use super::VectorMath;

#[ffi_export]
fn free_vec(v: Vector) {
    drop(v)
}

#[ffi_export]
fn new_vec(x: f64, y: f64, z: f64) -> Vector {
    (x, y, z).into()
}

#[ffi_export]
fn dup_vec(v: Vector) -> Vector {
    new_vec(v.x, v.y, v.z)
}

#[ffi_export]
fn fmt_vec(v: Vector) -> char_p_boxed {
    char_p::new(v.to_string())
}

#[ffi_export]
fn fmt_vec_ow(prev: &mut char_p_boxed, v: Vector) {
    *prev = CString::new(v.to_string()).unwrap().into();
}

#[ffi_export]
fn print_vec(v: Vector) {
    print!("{}", v);
}

#[ffi_export]
fn add_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = *u + v;
    u
}

#[ffi_export]
fn add_vec_new(u: &'_ mut Vector, v: Vector) -> Vector {
    *u + v
}

#[ffi_export]
fn sub_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = *u - v;
    u
}

#[ffi_export]
fn sub_vec_new(u: &'_ mut Vector, v: Vector) -> Vector {
    *u - v
}

#[ffi_export]
fn add_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    *u = *u + (x, y, z).into();
    u
}

#[ffi_export]
fn add_xyz_new(u: &'_ mut Vector, x: f64, y: f64, z: f64) -> Vector {
    *u + (x, y, z).into()
}

#[ffi_export]
fn sub_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    *u = *u - (x, y, z).into();
    u
}

#[ffi_export]
fn sub_xyz_new(u: &'_ mut Vector, x: f64, y: f64, z: f64) -> Vector {
    *u - (x, y, z).into()
}

#[ffi_export]
fn mul_vec_scalar<'x>(u: &'x mut Vector, multiplicand: f64) -> &'x mut Vector {
    *u = *u * multiplicand;
    u
}

#[ffi_export]
fn mul_vec_scalar_new(u: &'_ mut Vector, multiplicand: f64) -> Vector {
    *u * multiplicand
}

#[ffi_export]
fn div_vec_scalar<'x>(u: &'x mut Vector, dividand: f64) -> &'x mut Vector {
    *u = *u / dividand;
    u
}

#[ffi_export]
fn div_vec_scalar_new(u: &'_ mut Vector, dividand: f64) -> Vector {
    *u / dividand
}

#[ffi_export]
fn dotprod_vec<'x>(u: &'x mut Vector, v: Vector) -> f64 {
    u.dot_product(&v)
}

#[ffi_export]
fn crossprod_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = u.cross_product(&v);
    u
}

#[ffi_export]
fn crossprod_vec_new(u: &'_ Vector, v: Vector) -> Vector {
    u.cross_product(&v)
}

#[ffi_export]
fn magnitude_vec(u: &'_ Vector) -> f64 {
    u.magnitude()
}

#[ffi_export]
fn normalise_vec<'x>(u: &'x mut Vector) -> &'x mut Vector {
    *u = u.normalise();
    u
}

#[ffi_export]
fn normalise_vec_new(u: &'_ Vector) -> Vector {
    u.normalise()
}

#[ffi_export]
fn fast_normalise_vec<'x>(u: &'x mut Vector) -> &'x mut Vector {
    // uses the quake fast inverse square root approximation (newton's method)
    // not garanteed to be faster on modern cpus but could be useful in some cases
    *u = u.fast_normalise();
    u
}

#[ffi_export]
fn fast_normalise_vec_new(u: &'_ mut Vector) -> Vector {
    u.fast_normalise()
}

#[ffi_export]
fn angle(u: &'_ Vector, v: &'_ Vector) -> f64 {
    u.angle(&v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dup_vec() {
        let v = new_vec(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        let u = dup_vec(v);
        assert_eq!(u, v);
    }

    #[test]
    fn test_ops_vec() {
        let v = new_vec(1.0, 2.0, 3.0);
        let mut u = dup_vec(v);
        let result = new_vec(2.0, 4.0, 6.0);
        assert_eq!(*add_vec(&mut u, v), result);
        assert_eq!(*sub_vec(&mut u, v), v);
        assert_eq!(*mul_vec_scalar(&mut u, 2f64), result);
        assert_eq!(*div_vec_scalar(&mut u, 2f64), v);
        let u_norm = new_vec(0.2672612419124244, 0.5345224838248488, 0.8017837257372732);
        let mut u_cp = dup_vec(u);
        assert_eq!(*normalise_vec(&mut u), u_norm);
        println!("quake_normalised: {}", fast_normalise_vec(&mut u_cp));

        let x = new_vec(3.0, 6.0, 1.0);
        let y = new_vec(-5.0, -9.0, 4.0);
        println!(r#"angle : {}
||x|| = {}
||y|| = {}
x o y = {}
x̂ = {}
ŷ = {}"#, 
        x.angle(&y), x.magnitude(), y.magnitude(), x.dot_product(&y), x.normalise(), y.normalise());

    }
}
