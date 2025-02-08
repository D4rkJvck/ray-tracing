use {
    super::Vector,
    std::ops::{Div, Mul},
};

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Self::Output {
        Vector::new(self / rhs.x(), self / rhs.y(), self / rhs.z())
    }
}
