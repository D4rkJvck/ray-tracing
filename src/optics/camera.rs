use {
    super::Ray,
    crate::{
        Direction,
        Position,
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
};

pub struct Camera {
    origin:      Position,
    bottom_left: Position,
    horizontal:  Direction,
    vertical:    Direction,
}

impl Camera {
    pub fn new(origin: Position) -> Self {
        let width = IMAGE_WIDTH as f64 / 100.0;
        let height = IMAGE_HEIGTH as f64 / 100.0;

        let horizontal = Direction::new(width, 0.0, 0.0);
        let vertical = Direction::new(0.0, height, 0.0);

        let bottom_left = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Direction::new(0.0, 0.0, 1.0);

        Self {
            origin,
            bottom_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left + u * self.horizontal + v * self.vertical
                - self.origin,
        )
    }

    pub fn origin(&self) -> Position { self.origin }

    pub fn bottom_left(&self) -> Position { self.bottom_left }

    pub fn vertical(&self) -> Direction { self.vertical }

    pub fn horizontal(&self) -> Direction { self.horizontal }
}
