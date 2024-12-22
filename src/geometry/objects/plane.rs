use {
    super::Object,
    crate::{
        optics::Ray,
        Color,
        Position,
    },
};

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
            color: color.unit(),
        }
    }
}

impl Object for FlatPlane {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.position }

    fn hit(&self, ray: &Ray) -> f32 {
        let denominator = ray
            .direction()
            .dot(self.normal);

        if denominator.abs() > 1e-6 {
            let ray_to_plane = self.position - ray.origin();
            let t = ray_to_plane.dot(self.normal) / denominator;
            t
        }
        else {
            -1.0
        }
    }
}
