mod cube;
mod cylinder;
mod plane;
mod sphere;

use {
    super::{
        Color,
        Position,
    },
    crate::optics::Ray,
};
pub use {
    // cube::Cube,
    cylinder::Cylinder,
    plane::FlatPlane,
    sphere::Sphere,
};

pub trait Object {
    fn color(&self) -> Color;
    fn position(&self) -> Position;
    // fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool;
    fn hit(&self, ray: &Ray) -> f32;

    fn depth(&self) -> i32 { (self.position().z() * 1e6) as i32 }
}
