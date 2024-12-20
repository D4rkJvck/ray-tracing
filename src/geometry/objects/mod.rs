mod cube;
mod cylinder;
mod plane;
mod sphere;

pub use sphere::Sphere;
use {
    super::Color,
    crate::optics::Ray,
};

pub trait Object {
    fn color(&self) -> Color;
    fn got_hit_by(&self, ray: &Ray) -> bool;
}
