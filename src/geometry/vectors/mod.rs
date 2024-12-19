mod traits;

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn dot(&self, other: Self) -> f32 {
        let factor = *self * other;
        factor.x + factor.y + factor.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalized(&mut self) { *self /= self.magnitude() }

    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
}
