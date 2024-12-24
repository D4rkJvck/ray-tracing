use {
    crate::{
        geometry::Impact, Color, Direction, Object, Position
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

    pub fn color(&self, objects: &mut Vec<Box<dyn Object>>) -> Color {
        let mut impact = Impact::new();

        objects.sort_by_key(|object| -object.depth());

        for object in objects {
            if object.hit(self, 0.0, INFINITY, &mut impact) {
                return 0.5 * (impact.normal + Color::new(1.0, 1.0, 1.0))
            }
        }

        let t = 0.5 * (self.direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
            + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
