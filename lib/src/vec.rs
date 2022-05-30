use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use ::safer_ffi::prelude::*;
use safer_ffi::char_p::*;

#[derive_ReprC]
#[repr(C)]
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(t: (f64, f64, f64)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

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
fn print_vec(v: Vector) {
    print!("{}", v);
}

#[ffi_export]
fn add_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = *u + v;
    u
}

#[ffi_export]
fn sub_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = *u - v;
    u
}

#[ffi_export]
fn add_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    *u = *u + (x, y, z).into();
    u
}

#[ffi_export]
fn sub_xyz<'x>(u: &'x mut Vector, x: f64, y: f64, z: f64) -> &'x mut Vector {
    *u = *u - (x, y, z).into();
    u
}

#[ffi_export]
fn mul_vec<'x>(u: &'x mut Vector, multiplicand: f64) -> &'x mut Vector {
    *u = *u * multiplicand;
    u
}

#[ffi_export]
fn div_vec<'x>(u: &'x mut Vector, dividand: f64) -> &'x mut Vector {
    *u = *u / dividand;
    u
}

#[ffi_export]
fn dotprod_vec<'x>(u: &'x mut Vector, v: Vector) -> &'x mut Vector {
    *u = *u * v;
    u
}

#[ffi_export]
fn magnitude_vec(u: &'_ Vector) -> f64 {
    f64::sqrt(u.x * u.x + u.y * u.y + u.z * u.z)
}

#[ffi_export]
fn normalise_vec<'x>(u: &'x mut Vector) -> &'x mut Vector {
    let mag = magnitude_vec(u);
    *u = *u / mag;
    u
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
        assert_eq!(*mul_vec(&mut u, 2f64), result);
        assert_eq!(*div_vec(&mut u, 2f64), v);
        let u_norm = new_vec(0.2672612419124244, 0.5345224838248488, 0.8017837257372732);
        assert_eq!(*normalise_vec(&mut u), u_norm);
        // println!("Normalised: {}", u)
    }
}
