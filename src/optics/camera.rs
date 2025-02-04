use {
    super::Ray,
    crate::{
        common::ASPECT_RATIO,
        utils::degrees_to_radians,
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
    pub fn new(
        origin: Position,
        target: Position,
        view_up: Position,
        vertical_field_of_view: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_field_of_view);
        let h = f64::tan(theta / 2.);
        let viewport_height = h * 2.;
        let viewport_width = viewport_height * ASPECT_RATIO;

        let w = (origin - target).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let bottom_leftmost = origin - horizontal / 2. - vertical / 2. - focus_dist * w;

        let lens_radius = aperture / 2.;

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

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Direction::random_unit_lens();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.bottom_leftmost + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
