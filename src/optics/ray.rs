use {
    crate::{
        geometry::Impact,
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

    pub fn color(&self, objects: &Vec<Box<dyn Object>>) -> Color {
        let t = 0.5 * (self.direction.y() + 1.0);

        let mut ray_color = (1.0 - t) * Color::new(1.0, 1.0, 1.0)
            + t * Color::new(0.5, 0.7, 1.0);

        for object in objects {
            let mut impact = Impact::new();

            if object.hit(
                self,
                0.0,
                INFINITY,
                &mut impact,
            ) {
                ray_color =
                    0.5 * (impact.normal + Color::new(1.0, 1.0, 1.0));
            }
        }

        ray_color
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
