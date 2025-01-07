use {
    super::{
        Impact,
        Object,
    },
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
    pub fn new(position: Position, normal: Position, color: Color) -> Self {
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

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, impact: &mut Impact) -> bool {
        let denominator = ray.direction().dot(self.normal);

        if denominator.abs() <= 1e-6 {
            return false;
        }

        let ray_to_plane = self.position - ray.origin();
        let t = ray_to_plane.dot(self.normal) / denominator;

        if t < t_min || t > t_max {
            return false;
        }

        impact.t = t;
        impact.point = ray.cast(impact.t);
        impact.set_face_normal(ray.direction(), self.normal);
        true
    }
}
