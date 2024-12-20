use crate::{
    Color,
    Direction,
    Object,
    Position,
};

#[allow(unused)]
pub struct Ray {
    origin:    Position,
    direction: Direction,
}

#[allow(unused)]
impl Ray {
    pub fn new(origin: Position, mut direction: Direction) -> Self {
        direction.normalized();

        Self {
            origin,
            direction,
        }
    }

    pub fn cast(&self, t: f32) -> Position {
        self.origin + t * self.direction
    }

    pub fn color(&self, objects: &Vec<Box<dyn Object>>) -> Color {
        for object in objects {
            if object.got_hit_by(self) {
                return object.color();
            }
        }

        let t = 0.5 * (self.direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0)
            + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
