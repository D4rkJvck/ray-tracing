use {
    super::{
        Color,
        Vector,
    },
    crate::common::{
        clamp,
        SAMPLES_PER_PXL,
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
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / SAMPLES_PER_PXL as f64;

        r *= scale;
        g *= scale;
        b *= scale;

        write!(
            f,
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as i32,
            (256.0 * clamp(g, 0.0, 0.999)) as i32,
            (256.0 * clamp(b, 0.0, 0.999)) as i32,
        )
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

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, t: f64) { *self = *self * t; }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, t: f64) { *self = *self / t; }
}
