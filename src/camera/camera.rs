use crate::{renderer::Ray, Direction, Position, ASPECT_RATIO};

#[allow(unused)]
pub struct Camera {
    origin: Position,
    low_left_corner: Position,
    vertical: Direction,
    horizontal: Direction,
}

impl Camera {
    pub fn new(viewport_height: f32, focal_length: f32) -> Self {
        let viewport_width = viewport_height * ASPECT_RATIO;

        let origin = Position::default();
        let horizontal = Direction::new(viewport_width, 0.0, 0.0);
        let vertical = Direction::new(0.0, viewport_height, 0.0);
        let low_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Direction::new(0.0, 0.0, focal_length);

        Self {
            origin,
            low_left_corner,
            vertical,
            horizontal,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.low_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
