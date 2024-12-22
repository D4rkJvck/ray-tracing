mod vector_ops;
mod mutation;
mod scalar_ops;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vector(f32, f32, f32);

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self(x, y, z) }

    pub fn x(&self) -> f32 { self.0 }

    pub fn y(&self) -> f32 { self.1 }

    pub fn z(&self) -> f32 { self.2 }

    pub fn dot(&self, other: Self) -> f32 {
        let factor = *self * other;
        factor.x() + factor.y() + factor.z()
    }

    pub fn length(&self) -> f32 { self.dot(*self).sqrt() }

    pub fn unit(self) -> Self { self / self.length() }
}
