use {
    super::Object,
    crate::{
        optics::Ray,
        Color,
        Position,
    },
};

#[allow(unused)]
pub struct FlatPlane {
    plane:  Position,
    normal: Position,
    color:  Color,
}

impl FlatPlane {
    pub fn new(plane: Position, normal: Position, color: Color) -> Self {
        Self { plane,
               normal,
               color }
    }
}

impl Object for FlatPlane {
    fn color(&self) -> Color { self.color }

    fn got_hit_by(&self, ray: &Ray) -> bool {
        let denominator = ray.direction().dot(self.normal);

        if denominator.abs() > 1e-6 {
            let ray_to_plane = self.plane - ray.origin();
            let t = ray_to_plane.dot(self.normal) / denominator;
            t >= 0.0
        } else {
            false
        }
    }
}
