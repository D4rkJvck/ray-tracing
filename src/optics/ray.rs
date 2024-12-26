use {
    crate::{
        geometry::Impact,
        Color,
        Direction,
        Object,
        Position,
        optics::Light,  
    },
    std::f64::INFINITY,
};

pub struct Ray {
    origin: Position,
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

    pub fn color(
        &self,
        objects: &Vec<Box<dyn Object>>,
        lights: &Vec<Light>,
        depth: i32,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut closest_impact = Impact::new();
        let mut closest_object: Option<&Box<dyn Object>> = None;
        let mut closest_t = INFINITY;

        for object in objects {
            let mut impact = Impact::new();
            if object.hit(self, 0.001, closest_t, &mut impact) {
                closest_t = impact.t;
                closest_impact = impact;
                closest_object = Some(object);
            }
        }

        if let Some(object) = closest_object {
            let mut final_color = Color::new(0.0, 0.0, 0.0);
            let ambient = Color::new(0.1, 0.1, 0.1); // Ambient light

            // Add ambient light
            final_color += ambient * object.color();

            // Add contribution from each light
            for light in lights {
                let light_contribution = light.calculate_lighting(
                    closest_impact.point,
                    closest_impact.normal,
                );
                final_color += light_contribution * object.color();
            }

            final_color
        } else {
            let t = 0.5 * (self.direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn origin(&self) -> Position {
        self.origin
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}