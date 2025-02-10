use crate::{
    Direction,
    Position,
};

#[derive(Default, Clone, Copy)]
pub struct Impact {
    point:          Position,
    surface_normal: Direction,
    front_face:     bool,
}

impl Impact {
    pub fn new(
        point: Position,
        surface_normal: Direction,
        front_face: bool,
    ) -> Self {
        Self {
            point,
            surface_normal,
            front_face,
        }
    }

    pub fn set_face_normal(
        &mut self,
        incident: Direction,
        outward: Direction,
    ) {
        let cos_angle = incident.dot(outward);
        self.surface_normal =
            if cos_angle < 0. { outward } else { -outward }
    }

    pub fn point(&self) -> Position { self.point }

    pub fn surface_normal(&self) -> Direction { self.surface_normal }

    pub fn front_face(&self) -> bool { self.front_face }
}
