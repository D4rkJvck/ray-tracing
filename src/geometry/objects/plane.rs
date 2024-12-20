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
    position: Position,
    normal:   Position,
    color:    Color,
}

impl FlatPlane {
    pub fn new(
        position: Position,
        normal: Position,
        color: Color,
    ) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }
}

impl Object for FlatPlane {
    fn color(&self) -> Color { self.color }

    fn got_hit_by(&self, ray: &Ray) -> bool {
        let denominator = ray
            .direction()
            .dot(self.normal);

        if denominator.abs() > 1e-6 {
            let ray_to_plane = self.position - ray.origin();
            let t = ray_to_plane.dot(self.normal) / denominator;
            t >= 0.0
        }
        else {
            false
        }
    }
}
