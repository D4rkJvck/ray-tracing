use super::{Color, Vector};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.x(), -self.y(), -self.z()) }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) { *self = *self + other; }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, t: f32) { *self = *self * t; }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, t: f32) { *self = *self / t; }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output { Self::new(self.x() * t, self.y() * t, self.z() * t) }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}
