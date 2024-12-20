mod cube;
mod cylinder;
mod plane;
mod sphere;

use {
    super::Color,
    crate::optics::Ray,
};
pub use {
    plane::FlatPlane,
    sphere::Sphere,
};

pub trait Object {
    fn color(&self) -> Color;
    fn got_hit_by(&self, ray: &Ray) -> bool;
}
