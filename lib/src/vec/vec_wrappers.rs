use ::safer_ffi::prelude::*;
use safer_ffi::char_p::char_p_boxed;
use std::ffi::CString;

use super::Vector;
use super::VectorMath;

#[ffi_export]
fn free_vec(v: Vector) {
    drop(v);
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
fn add_vec(u: Vector, v: Vector) -> Vector {
    u + v
}

#[ffi_export]
fn sub_vec(u: Vector, v: Vector) -> Vector {
    u - v
}

#[ffi_export]
fn add_xyz(u: Vector, x: f64, y: f64, z: f64) -> Vector {
    u + (x, y, z).into()
}

#[ffi_export]
fn sub_xyz(u: Vector, x: f64, y: f64, z: f64) -> Vector {
    u - (x, y, z).into()
}

#[ffi_export]
fn mul_vec_scalar(u: Vector, multiplicand: f64) -> Vector {
    u * multiplicand
}

#[ffi_export]
fn div_vec_scalar(u: Vector, dividand: f64) -> Vector {
    u / dividand
}

#[ffi_export]
fn dotprod_vec(u: Vector, v: Vector) -> f64 {
    u.scalar_product(&v)
}

#[ffi_export]
fn crossprod_vec(u: Vector, v: Vector) -> Vector {
    u.cross_product(&v)
}

#[ffi_export]
fn magnitude_vec(u: Vector) -> f64 {
    u.magnitude()
}

#[ffi_export]
fn normalise_vec(u: Vector) -> Vector {
    u.normalise()
}

#[ffi_export]
fn fast_normalise_vec(u: Vector) -> Vector {
    u.fast_normalise()
}

#[ffi_export]
fn angle_vec(u: Vector, v: Vector) -> f64 {
    u.angle(&v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup_vec() {
        let v = new_vec(1.0, 2.0, 3.0);
        let error_margin = f64::EPSILON;
        assert!(v.x - 1.0 < error_margin);
        assert!(v.y - 2.0 < error_margin);
        assert!(v.x - 3.0 < error_margin);
        let u = dup_vec(v);
        assert_eq!(u, v);
    }

    #[test]
    fn test_ops_vec() {
        let v = new_vec(1.0, 2.0, 3.0);
        let u = dup_vec(v);
        let result = new_vec(2.0, 4.0, 6.0);
        assert_eq!(add_vec(u, v), result);
        assert_eq!(sub_vec(u, v), crate::vec::origin());
        assert_eq!(mul_vec_scalar(u, 2f64), result);
        assert_eq!(div_vec_scalar(result, 2f64), v);
        let u_norm = new_vec(0.267_261_241_912_424_4, 0.534_522_483_824_848_8, 0.801_783_725_737_273_2);
        let u_cp = dup_vec(u);
        assert_eq!(normalise_vec(u), u_norm);

        let x = new_vec(3.0, 6.0, 1.0);
        let y = new_vec(-5.0, -9.0, 4.0);
        println!(r#"angle : {}
||x|| = {}
||y|| = {}
x o y = {}
x^ = {}
y^ = {}"#, 
        angle_vec(x, y), 
        magnitude_vec(x), 
        magnitude_vec(y), 
        dotprod_vec(x, y), 
        normalise_vec(x), 
        normalise_vec(y));
        println!("quake_normalised: {}", fast_normalise_vec(u_cp));
    }
}
