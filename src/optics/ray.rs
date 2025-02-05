use {
    super::Impact,
    crate::{
        optics::Light,
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

        Impact::new(point, surface_normal, t)
    }

    pub fn color(
        &self,
        objects: &Vec<Box<dyn Object>>,
        lights: &Vec<Light>,
        max_depth: i32,
    ) -> Color {
        if max_depth <= 0 {
            return Color::default();
        }

        // let mut closest_impact = Impact::new();
        // let mut closest_object: Option<&Box<dyn Object>> = None;
        // let mut closest_t = INFINITY;

        for object in objects {
            if let Some(impact) = object.hit(
                self, 0.001, // closest_t,
                INFINITY,
            ) {
                // let direction =
                //     impact.surface_normal + Direction::random_unit();
                if let Some((attenuation, scattered)) = object
                    .material()
                    .scatter(self, &impact)
                {
                    return attenuation
                        * scattered.color(objects, lights, max_depth - 1);
                }
                // closest_t = impact.t;
                // closest_impact = impact;
                // closest_object = Some(object);
            }
        }

        // if let Some(object) = closest_object {
        //     let mut final_color = Color::default();
        //     let ambient = Color::new(0.1, 0.1, 0.1); // Ambient light

        //     // Add ambient light
        //     final_color += ambient * object.color();

        //     // Add contribution from each light
        //     for light in lights {
        //         let illumination = light.illuminate(&closest_impact,
        // objects);         final_color += illumination *
        // object.color();     }

        //     final_color
        // }
        // else {
        let t = 0.5 * (self.direction.y() + 1.);
        (1. - t) * Color::new(1., 1., 1.)
            + t * Color::new(0.5, 0.7, 1.)
        // }
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
