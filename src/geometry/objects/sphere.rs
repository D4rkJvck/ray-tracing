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

pub struct Sphere {
    center: Position,
    radius: f64,
    color:  Color,
}

impl Sphere {
    pub fn new(center: Position, radius: f64, color: Color) -> Self {
        Self {
            center,
            radius,
            color: color.unit(),
        }
    }
}

impl Object for Sphere {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.center }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let direction = ray.direction();
        let distance = ray.origin() - self.center;

        let a = direction.dot(direction);
        let h = direction.dot(distance);
        let c = distance.dot(distance) - self.radius.powf(2.0);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-h - discriminant.sqrt()) / a;

        if root <= t_min || t_max <= root {
            root = (-h + discriminant.sqrt()) / a;

            if root <= t_min || t_max <= root {
                return None;
            };
        };

        let outward = (ray.cast(root) - self.center) / self.radius;
        
        Some(ray.generate_impact(outward, root))
    }
}
