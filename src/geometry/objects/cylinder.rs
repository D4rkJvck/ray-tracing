use {
    super::{
        Impact,
        Object,
    },
    crate::{
        optics::Ray,
        Color,
        Direction,
        Position,
    },
};

pub struct Cylinder {
    center:      Position,
    radius:      f64,
    height:      f64,
    orientation: Direction,
    color:       Color,
}

impl Cylinder {
    pub fn new(
        center: Position,
        radius: f64,
        height: f64,
        orientation: Direction,
        color: Color,
    ) -> Self {
        Self {
            center,
            radius,
            height,
            orientation: orientation.unit(),
            color: color.unit(),
        }
    }
}

impl Object for Cylinder {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.center }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let oc = ray.origin() - self.center;

        // Calcul pour la surface latérale
        let dot_dir_orient = ray
            .direction()
            .dot(self.orientation);
        let dot_oc_orient = oc.dot(self.orientation);

        let a = ray
            .direction()
            .dot(ray.direction())
            - dot_dir_orient * dot_dir_orient;
        if a.abs() < 1e-6 {
            return None;
        }

        let b = 2.0
            * (ray.direction().dot(oc) - dot_dir_orient * dot_oc_orient);
        let c = oc.dot(oc)
            - dot_oc_orient * dot_oc_orient
            - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_min || t > t_max {
                return None;
            }
        }

        let hit_point = ray.cast(t);
        let height_vec = hit_point - self.center;
        let height = height_vec.dot(self.orientation);

        if height < 0.0 || height > self.height {
            return None;
        }

        let projected = self.center + height * self.orientation;
        let normal = (hit_point - projected).unit();

        Some(ray.generate_impact(normal, t))
    }
}
