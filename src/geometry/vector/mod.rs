use crate::common::{
    random_double,
    random_double_range,
};

mod mutation;
mod scalar_ops;
mod vector_ops;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vector(f64, f64, f64);

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self(x, y, z) }

    pub fn random() -> Self {
        Self(
            random_double(),
            random_double(),
            random_double(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn x(&self) -> f64 { self.0 }

    pub fn y(&self) -> f64 { self.1 }

    pub fn z(&self) -> f64 { self.2 }

    pub fn dot(&self, other: Self) -> f64 {
        let factor = *self * other;
        factor.x() + factor.y() + factor.z()
    }

    pub fn length(&self) -> f64 { self.dot(*self).sqrt() }

    pub fn unit(self) -> Self { self / self.length() }
}
