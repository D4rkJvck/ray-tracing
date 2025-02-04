use {
    super::Camera,
    crate::{
        common::ASPECT_RATIO,
        error::Error,
        utils::degrees_to_radians,
        Position,
        Result,
    },
};

pub struct CameraBuilder {
    origin:     Position,
    target:     Position,
    vup:        Position,
    vfov:       f64,
    aperture:   f64,
    focus_dist: Option<f64>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            origin:     Position::default(),
            target:     Position::new(0., 0., -1.),
            vup:        Position::new(0., 1., 0.),
            vfov:       90.,
            aperture:   0.1,
            focus_dist: None,
        }
    }
}

impl CameraBuilder {
    pub fn origin(mut self, look_from: Position) -> Self {
        self.origin = look_from;
        self
    }

    pub fn target(mut self, look_at: Position) -> Self {
        self.target = look_at;
        self
    }

    pub fn view_up(mut self, up: Position) -> Self {
        self.vup = up;
        self
    }

    pub fn vertical_field_of_view(mut self, degrees: f64) -> Self {
        self.vfov = degrees;
        self
    }

    pub fn aperture(mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_distance(mut self, distance: f64) -> Self {
        self.focus_dist = Some(distance);
        self
    }

    pub fn build(self) -> Result<Camera> {
        let focus_dist = self
            .focus_dist
            .unwrap_or_else(|| (self.origin - self.target).length());

        if focus_dist <= 0. {
            return Err(Error::InvalidCamera(
                "Focus distance must be positive",
            ));
        }

        if self.aperture < 0. {
            return Err(Error::InvalidCamera(
                "Aperture must be non-negative",
            ));
        }

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = h * 2.;
        let viewport_width = viewport_height * ASPECT_RATIO;

        let w = (self.origin - self.target).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let bottom_leftmost =
            self.origin - horizontal / 2. - vertical / 2. - focus_dist * w;

        let lens_radius = self.aperture / 2.;

        Ok(Camera::new(
            self.origin,
            horizontal,
            vertical,
            bottom_leftmost,
            u,
            v,
            lens_radius,
        ))
    }
}
