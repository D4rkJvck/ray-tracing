use {
    super::Ray,
    crate::{
        Direction,
        Position,
        ASPECT_RATIO,
    },
};

#[allow(unused)]
pub struct Camera {
    origin:      Position,
    bottom_left: Position,
    horizontal:  Direction,
    vertical:    Direction,
}

impl Camera {
    pub fn new(origin: Position,
               viewport_height: f32,
               focal_length: f32)
               -> Self {
        let viewport_width = viewport_height * ASPECT_RATIO;

        let horizontal = Direction::new(viewport_width, 0.0, 0.0);
        let vertical = Direction::new(0.0, viewport_height, 0.0);
        let bottom_left = origin
                          - horizontal / 2.0
                          - vertical / 2.0
                          - Direction::new(0.0, 0.0, focal_length);

        Self { origin,
               bottom_left,
               horizontal,
               vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.bottom_left
                 + u * self.horizontal
                 + v * self.vertical
                 - self.origin)
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn bottom_left(&self) -> Position { self.bottom_left }

    pub fn vertical(&self) -> Direction { self.vertical }

    pub fn horizontal(&self) -> Direction { self.horizontal }
}
