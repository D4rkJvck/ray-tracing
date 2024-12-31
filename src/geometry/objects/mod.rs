mod cube;
mod cylinder;
mod plane;
mod sphere;

use {
    super::{
        Color,
        Direction,
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, impact: &mut Impact) -> bool;
    fn depth(&self) -> i32 { (self.position().z() * 1e6) as i32 }
}

#[derive(Default, Clone)]
pub struct Impact {
    pub point:          Position,
    pub surface_normal: Direction,
    pub t:              f64,
    // pub front_face: bool,
}

impl Impact {
    pub fn new() -> Self { Self::default() }

    pub fn set_face_normal(&mut self, incident: Direction, outward: Direction) {
        let cos_angle = incident.dot(outward);
        self.surface_normal = if cos_angle < 0.0 { outward } else { -outward }
    }
}
