use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use ::safer_ffi::prelude::*;
use fast_inv_sqrt::InvSqrt64;

mod vec_wrappers;

pub trait VectorMath {
    fn scalar_product(&self, other_vec: &Self) -> f64;
    fn cross_product(&self, other_vec: &Self) -> Self;
    fn magnitude(&self) -> f64;
    fn normalise(&self) -> Self;
    fn fast_normalise(&self) -> Self;
    fn angle(&self, other_vec: &Self) -> f64;
}

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

impl From<Vector> for (f64, f64, f64) {
    fn from(v: Vector) -> Self {
        (v.x, v.y, v.z)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, addend: Vector) -> Vector {
        Vector {
            x: self.x + addend.x,
            y: self.y + addend.y,
            z: self.z + addend.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, subtrahend: Vector) -> Vector {
        Vector {
            x: self.x - subtrahend.x,
            y: self.y - subtrahend.y,
            z: self.z - subtrahend.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, multiplicand: f64) -> Vector {
        Vector {
            x: self.x * multiplicand,
            y: self.y * multiplicand,
            z: self.z * multiplicand,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, denominator: f64) -> Vector {
        Vector {
            x: self.x / denominator,
            y: self.y / denominator,
            z: self.z / denominator,
        }
    }
}

impl VectorMath for Vector {
    fn scalar_product(&self, other_vec: &Self) -> f64 {
        self.x * other_vec.x + self.y * other_vec.y + self.z * other_vec.z
    }

    fn cross_product(&self, other_vec: &Self) -> Self {
        Vector {
            x: self.y * other_vec.z - self.z * other_vec.y,
            y: self.z * other_vec.x - self.x * other_vec.z,
            z: self.x * other_vec.y - self.y * other_vec.x,
        }
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self.scalar_product(self))
    }

    fn normalise(&self) -> Self {
        *self / self.magnitude()
    }

    fn fast_normalise(&self) -> Self {
        *self * (self.scalar_product(self)).inv_sqrt64()
    }

    fn angle(&self, other_vec: &Self) -> f64 {
        self.scalar_product(other_vec)
            .div(self.magnitude() * other_vec.magnitude())
            .acos()
            .to_degrees()
    }
}

#[ffi_export]
fn i() -> Vector {
    Vector { x: 1.0, y: 0.0, z: 0.0 }
}

#[ffi_export]
fn j() -> Vector {
    Vector { x: 0.0, y: 1.0, z: 0.0 }
}

#[ffi_export]
fn k() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 1.0 }
}
    
#[ffi_export]
fn origin() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 0.0 }
}
