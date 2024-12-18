use super::Vector;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign,
        Neg, Sub,
    },
};

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, t: f64) { *self = *self * t; }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, t: f64) { *self = *self / t; }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z(),
        )
    }
}
