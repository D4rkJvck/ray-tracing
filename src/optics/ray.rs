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
        let direction = direction.normal();

        Self {
            origin,
            direction,
        }
    }

    pub fn cast(&self, t: f32) -> Position { self.origin + t * self.direction }

    pub fn color(&self, objects: &Vec<Box<dyn Object>>) -> Color {
        for object in objects {
            let t = object.hit(self);

            if t > 0.0 {
                let n = self.cast(t).normal() - Direction::new(0.0, 0.0, -1.0);
                return 0.5
                    * Color::new(
                        n.x() + 1.0,
                        n.y() + 1.0,
                        n.z() + 1.0,
                    );
            }
        }

        let t = 0.5 * (self.direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn direction(&self) -> Direction { self.direction }
}
