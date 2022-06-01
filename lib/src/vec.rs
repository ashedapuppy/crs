use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use ::safer_ffi::prelude::*;
use fast_inv_sqrt::InvSqrt64;

mod vec_wrappers;

pub trait VectorMath {
    fn dot_product(&self, other: &Self) -> f64;
    fn dot_product_self(&self) -> f64;
    fn cross_product(&self, other: &Self) -> Self;
    fn magnitude(&self) -> f64;
    fn normalise(&self) -> Self;
    fn fast_normalise(&self) -> Self;
    fn angle(&self, other: &Self) -> f64;
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

impl VectorMath for Vector {
    fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn dot_product_self(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn cross_product(&self, other: &Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt(self.dot_product_self())
    }

    fn normalise(&self) -> Self {
        *self / self.magnitude()
    }

    fn fast_normalise(&self) -> Self {
        *self * (self.dot_product_self()).inv_sqrt64()
    }

    fn angle(&self, other: &Self) -> f64 {
        self
        .dot_product(other)
        .div(self.magnitude() * other.magnitude())
        .acos()
        .to_degrees()
    }

}

#[allow(dead_code)]
fn i() -> Vector {
Vector { x: 1.0, y: 0.0, z: 0.0 }
}

#[allow(dead_code)]
fn j() -> Vector {
Vector { x: 0.0, y: 1.0, z: 0.0 }
}

#[allow(dead_code)]
fn k() -> Vector {
Vector { x: 0.0, y: 0.0, z: 1.0 }
}
    
#[allow(dead_code)]
fn origin() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 0.0 }
}