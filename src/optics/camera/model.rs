use {
    super::builder::CameraBuilder,
    crate::{
        optics::Ray,
        Direction,
        Position,
    },
};

pub struct Camera {
    origin:          Position,
    horizontal:      Direction,
    vertical:        Direction,
    bottom_leftmost: Position,
    u:               Direction,
    v:               Direction,
    lens_radius:     f64,
}

impl Camera {
    pub(super) fn new(
        origin: Position,
        horizontal: Direction,
        vertical: Direction,
        bottom_leftmost: Position,
        u: Direction,
        v: Direction,
        lens_radius: f64,
    ) -> Self {
        Self {
            origin,
            horizontal,
            vertical,
            bottom_leftmost,
            u,
            v,
            lens_radius,
        }
    }

    pub fn builder() -> CameraBuilder { CameraBuilder::default() }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Direction::random_unit_lens();
        let offset = self.u * rd.x() + self.v * rd.y();
        
        Ray::new(
            self.origin + offset,
            self.bottom_leftmost + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        )
    }
}
