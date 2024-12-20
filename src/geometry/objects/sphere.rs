use {
    super::Object,
    crate::{
        optics::Ray,
        Color,
        Position,
    },
};

#[allow(unused)]
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
            color,
        }
    }
}

impl Object for Sphere {
    fn color(&self) -> Color { self.color }

    fn got_hit_by(&self, ray: &Ray) -> bool {
        let direction = ray.direction();
        let distance = ray.origin() - self.center;

        let a = direction.dot(direction);
        let b = 2.0 * direction.dot(distance);
        let c = distance.dot(distance) - self.radius.powf(2.0);

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
