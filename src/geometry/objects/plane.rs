use {
    super::Object,
    crate::{
        material::Material,
        optics::{Impact, Ray},
        Direction, Position,
    },
};

pub struct Plane {
    position: Position,
    normal: Direction,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(position: Position, normal: Direction, material: Box<dyn Material>) -> Self {
        Self {
            position,
            normal,
            material,
        }
    }
}

impl Object for Plane {
    fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    fn position(&self) -> Position {
        self.position
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let denominator = ray.direction().dot(self.normal);

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
