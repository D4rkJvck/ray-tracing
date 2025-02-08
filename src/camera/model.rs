use {
    super::builder::CameraBuilder,
    crate::{optics::Ray, Direction, Position},
};

pub struct Camera {
    origin: Position,
    horizontal: Direction,
    vertical: Direction,
    bottom_leftmost: Position,
    u: Direction,
    v: Direction,
    lens_radius: f64,
}

impl Camera {
    pub(super) fn new(
        params: (
            Position,
            Direction,
            Direction,
            Position,
            Direction,
            Direction,
            f64,
        ),
    ) -> Self {
        Self {
            origin: params.0,
            horizontal: params.1,
            vertical: params.2,
            bottom_leftmost: params.3,
            u: params.4,
            v: params.5,
            lens_radius: params.6,
        }
    }

    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Direction::random_unit_lens();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.bottom_leftmost + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    pub fn origin(&self) -> Position {
        self.origin
    }
}
