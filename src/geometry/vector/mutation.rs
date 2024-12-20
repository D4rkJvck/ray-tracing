use {
    super::{
        Color,
        Vector,
    },
    std::{
        fmt::{
            Display,
            Formatter,
            Result,
        },
        ops::{
            AddAssign,
            DivAssign,
            MulAssign,
            Neg,
        },
    },
};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let r = (255.999 * self.x()) as i32;
        let g = (255.999 * self.y()) as i32;
        let b = (255.999 * self.z()) as i32;

        write!(f, "{} {} {}", r, g, b)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
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
