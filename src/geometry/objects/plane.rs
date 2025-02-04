use {
    super::Object,
    crate::{
        optics::{
            Impact,
            Ray,
        },
        Color,
        Position,
    },
};

pub struct Plane {
    position: Position,
    normal:   Position,
    color:    Color,
}

impl Plane {
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

impl Object for Plane {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.position }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let denominator = ray
            .direction()
            .dot(self.normal);

        if denominator.abs() <= 1e-6 {
            return None;
        }

        let ray_to_plane = self.position - ray.origin();
        let t = ray_to_plane.dot(self.normal) / denominator;

        if t < t_min || t > t_max {
            return None;
        }

        Some(ray.generate_impact(self.normal, t))
    }
}
