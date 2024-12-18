mod traits;

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0)
            + self.y.powf(2.0)
            + self.z.powf(2.0))
        .sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.magnitude()
    }

    pub(super) fn x(&self) -> f64 { self.x }
    pub(super) fn y(&self) -> f64 { self.y }
    pub(super) fn z(&self) -> f64 { self.z }
}
