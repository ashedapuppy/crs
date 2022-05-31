use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use ::safer_ffi::prelude::*;

mod vec_simple_ops;

trait DotProduct {
    fn dot_product(&self, other: &Self) -> f64;
    fn dot_product_self(&self) -> f64;
}

trait CrossProduct {
    fn cross_product(&self, other: &Self) -> Self;
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

impl DotProduct for Vector {
    fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn dot_product_self(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl CrossProduct for Vector {
    fn cross_product(&self, other: &Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
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
