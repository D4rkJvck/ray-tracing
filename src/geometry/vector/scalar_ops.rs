use {
    super::Vector,
    std::ops::{
        Div,
        Mul,
    },
};

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
        )
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z(),
        )
    }
}
