use {
    super::Ray,
    crate::{
        common::{
            degrees_to_radians,
            ASPECT_RATIO,
        },
        Direction,
        Position,
    },
};

pub struct Camera {
    origin:          Position,
    // target:          Position,
    // view_up:         Direction,
    horizontal:      Direction,
    vertical:        Direction,
    bottom_leftmost: Position,
    // focal_length:    f64,
}

impl Camera {
    pub fn new(
        origin: Position,
        target: Position,
        view_up: Direction,
        vertical_field_of_view: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_field_of_view);
        let h = f64::tan(theta / 2.);
        let viewport_height = h * 2.;
        let viewport_width = viewport_height * ASPECT_RATIO;

        let w = (origin - target).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let bottom_leftmost = origin - horizontal / 2. - vertical / 2. - w;

        Self {
            origin,
            // target,
            // view_up,
            horizontal,
            vertical,
            bottom_leftmost,
            // focal_length: 1.,
        }
    }

    pub fn get_ray(&self, s: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_leftmost + s * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
