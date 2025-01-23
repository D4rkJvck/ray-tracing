use crate::{
    Direction,
    Position,
};

#[derive(Default, Clone, Copy)]
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
