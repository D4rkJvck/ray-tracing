use {
    super::Object,
    crate::{
        optics::Ray,
        Color,
        Position,
    },
};

pub struct Sphere {
    center: Position,
    radius: f32,
    color:  Color,
}

impl Sphere {
    pub fn new(center: Position, radius: f32, color: Color) -> Self {
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

    fn hit(&self, ray: &Ray) -> f32 {
        let direction = ray.direction();
        let distance = ray.origin() - self.center;

        let a = direction.dot(direction);
        let h = direction.dot(distance);
        let c = distance.dot(distance) - self.radius.powf(2.0);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        let root = (-h - discriminant.sqrt()) / a;
        // let mut root = (-h - discriminant.sqrt()) / a;

        // if root <= t_min || t_max <= root {
        //     root = (-h + discriminant.sqrt()) / a;

        //     if root <= t_min || t_max <= root {
        //         return false;
        //     };
        // };

        root
    }
}
