mod cube;
mod cylinder;
mod plane;
mod sphere;

use {
    super::Position,
    crate::{
        material::Material,
        optics::{Impact, Ray},
    },
};
pub use {cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere};

pub trait Object {
    fn material(&self) -> &dyn Material;
    fn position(&self) -> Position;
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact>;
    fn depth(&self) -> i32 {
        (self.position().z() * 1e6) as i32
    }
}
