mod arithmetics;
mod mutation;
mod scalar_ops;

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector(f32, f32, f32);

pub type Color = Vector;
pub type Position = Vector;
pub type Direction = Vector;

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self(x, y, z) }

    pub fn x(&self) -> f32 { self.0 }

    pub fn y(&self) -> f32 { self.1 }

    pub fn z(&self) -> f32 { self.2 }

    // Changed to public
    pub fn magnitude(&self) -> f32 { self.dot(*self).sqrt() }

    pub fn dot(&self, other: Self) -> f32 {
        let factor = *self * other;
        factor.x() + factor.y() + factor.z()
    }

    pub fn normal(self) -> Self { self / self.magnitude() }
}