use {
    super::{
        utils::{compute, validate_params},
        Camera,
    },
    crate::{Direction, Position, Result},
};

pub struct CameraBuilder {
    origin: Position,
    target: Position,
    vup: Direction,
    vfov: f64,
    aperture: f64,
    focus_dist: Option<f64>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            origin: Position::default(),
            target: Position::new(0., 0., -1.),
            vup: Direction::new(0., 1., 0.),
            vfov: 90.,
            aperture: 0.1,
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

    pub fn view_up(mut self, up: Direction) -> Self {
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

        validate_params(focus_dist, self.aperture, self.vfov)?;

        let params = compute(
            self.origin,
            self.target,
            self.vup,
            self.vfov,
            focus_dist,
            self.aperture,
        );

        Ok(Camera::new(params))
    }
}
