mod cylinder;
mod plane;
mod sphere;


// use crate::{
//     Color,
//     optics::Ray,
// };
use {
    super::Color,
    crate::optics::Ray,
};

pub use {
    cylinder::Cylinder,
    plane::FlatPlane,
    sphere::Sphere,
};

pub trait Object {
    fn color(&self) -> Color;
    fn hit(&self, ray: &Ray) -> f32;
}