use {
    super::{Impact, Object},
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

    fn hit(&self, ray: &Ray, _t_min: f64, _t_max: f64, impact: &mut Impact) -> bool {
        let denominator = ray
            .direction()
            .dot(self.normal);

        if denominator.abs() <= 1e-6 {
            return false
        }

        // let ray_to_plane = self.position - ray.origin();
        // let t = ray_to_plane.dot(self.normal) / denominator;

        true
    }
}
