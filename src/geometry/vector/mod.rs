mod mutation;
mod scalar_ops;
mod vector_ops;

use crate::common::{random_double, random_double_range};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vector(f64, f64, f64);

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

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

    pub fn random_unit() -> Self {
        Self::random_unit_sphere()
    }

    fn random_unit_sphere() -> Self {
        loop {
            let vec = Self::random_range(-1.0, 1.0);

            if vec.dot(vec) < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit_lens() -> Self {
        loop {
            let p = Self::new(
                random_double_range(-1., 1.),
                random_double_range(-1., 1.),
                0.,
            );

            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn reflect(self, other: Self) -> Self {
        self - 2. * self.dot(other) * other
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn near_0(&self) -> bool {
        const EPS: f64 = 1.0e-8;

        self.x().abs() < EPS
            && self.y().abs() < EPS
            && self.z().abs() < EPS
    }

    pub fn dot(&self, other: Self) -> f64 {
        let factor = *self * other;
        factor.x() + factor.y() + factor.z()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}
