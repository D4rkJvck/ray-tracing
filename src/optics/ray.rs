use {
    super::Impact,
    crate::{
        utils::BRIGHTNESS,
        Color,
        Direction,
        Object,
        Position,
    },
    std::f64::INFINITY,
};

pub struct Ray {
    origin:    Position,
    direction: Direction,
}

impl Ray {
    pub fn new(origin: Position, direction: Direction) -> Self {
        Self {
            origin,
            direction: direction.unit(),
        }
    }

    pub fn cast(&self, t: f64) -> Position {
        self.origin + t * self.direction
    }

    pub fn generate_impact(&self, outward: Direction, t: f64) -> Impact {
        let point = self.cast(t);
        let cos_angle = self.direction().dot(outward);
        let surface_normal =
            if cos_angle < 0. { outward } else { -outward };

        let mut impact = Impact::new(point, surface_normal, true);
        impact.set_face_normal(self.direction(), outward);
        impact
    }

    pub fn color(
        &self,
        objects: &Vec<Box<dyn Object>>,
        max_depth: i32,
    ) -> Color {
        if max_depth <= 0 {
            return Color::default();
        }

        for object in objects {
            if let Some(impact) = object.hit(self, 0.001, INFINITY) {
                let emission = object.material().emit();

                match object
                    .material()
                    .scatter(self, &impact)
                {
                    Some((attenuation, scattered)) => {
                        let scattered_color =
                            scattered.color(objects, max_depth - 1);

                        if scattered_color == Color::default() {
                            return emission;
                        }
                        return emission + attenuation * scattered_color;
                    }
                    None => return emission,
                }
            }
        }

        let t = 0.5 * (self.direction.y() + 1.);
        let base_color = (1. - t) * Color::new(0.01, 0.01, 0.01)
            + t * Color::new(0.005, 0.007, 0.01);

        base_color * BRIGHTNESS
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
