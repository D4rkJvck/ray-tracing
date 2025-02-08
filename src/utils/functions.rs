use {
    super::{
        Error,
        Result,
        ASPECT_RATIO,
    },
    crate::{
        Direction,
        Position,
    },
    rand::Rng,
    std::{
        f64::consts::PI,
        io,
    },
};

pub fn degrees_to_radians(degrees: f64) -> f64 { degrees * PI / 180.0 }

pub fn random_double() -> f64 { rand::rng().random() }

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}

pub fn validate_params(
    focus_dist: f64,
    aperture: f64,
    vfov: f64,
) -> Result<()> {
    if focus_dist <= 0. {
        return Err(Error::InvalidCamera(
            "Focus distance must be positive",
        ));
    };

    if aperture < 0. {
        return Err(Error::InvalidCamera(
            "Aperture must be non-negative",
        ));
    };

    if !(0.0..=180.).contains(&vfov) {
        return Err(Error::InvalidCamera(
            "Vertical FOV must be between 0 and 180 degrees",
        ));
    };

    Ok(())
}

pub fn compute(
    origin: Position,
    target: Position,
    vup: Direction,
    vfov: f64,
    focus_dist: f64,
    aperture: f64,
) -> (
    Position,
    Direction,
    Direction,
    Position,
    Direction,
    Direction,
    f64,
) {
    let theta = degrees_to_radians(vfov);
    let h = f64::tan(theta / 2.);
    let viewport_height = h * 2.;
    let viewport_width = viewport_height * ASPECT_RATIO;

    let w = (origin - target).unit();
    let u = vup.cross(w).unit();
    let v = w.cross(u);

    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let bottom_leftmost =
        origin - horizontal / 2. - vertical / 2. - focus_dist * w;

    let lens_radius = aperture / 2.;

    (
        origin,
        horizontal,
        vertical,
        bottom_leftmost,
        u,
        v,
        lens_radius,
    )
}
