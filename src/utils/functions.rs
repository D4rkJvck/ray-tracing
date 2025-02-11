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
        env,
        f64::consts::PI,
    },
};

//------------------------------------------------------------------------------------------------------------------------------------------------

fn degrees_to_radians(degrees: f64) -> f64 { degrees * PI / 180.0 }

fn get_brightness_from_args() -> f64 {
    env::args()
        .collect::<Vec<String>>()
        .iter()
        .find(|arg| arg.starts_with("--brightness="))
        .unwrap_or(&String::new())
        .split('=')
        .last()
        .unwrap_or("100")
        .parse::<f64>()
        .unwrap_or(100.)
}

pub fn init() -> f64 {
    let brightness = get_brightness_from_args();

    println!("\nRAY📸 🟦 🌐📽️ TRACING💡🎥🌟\n");
    println!(
        "The scene brightness will be set at: {}%\n ",
        brightness
    );

    println!("Choose one of these scenes:");
    println!("Scene 1: 1 sphere");
    println!("Scene 2: 1 flat plane + 1 cube");
    println!("Scene 3: 1 flat plane + 1 cylinder + 1 cube + 1 sphere");
    println!("Scene 4: scene 3 with the camera in another position\n");

    brightness
}

pub fn random_double() -> f64 { rand::rng().random() }

pub fn random_double_range(min: f64, max: f64) -> f64 { min + (max - min) * random_double() }

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}

/// 1. Checks that the focus distance is a positive number.
/// 2. Checks that the aperture is not a negative number.
/// 3. Makes sure the vertical field of view is within 0 and 180 degrees.
pub fn validate_params(focus_dist: f64, aperture: f64, vfov: f64) -> Result<()> {
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

/// 1. Viewport: it converts the vertical field of view into radians, then
///    retrieves the viewport's half-height from the vertical field of view, and
///    finally gets the viewport dimensions using its half-height and the aspect
///    ratio constant.
///
/// 2. Coordinate Sytem: it gets the forward direction from the origin and
///    target of the camera. It then retrieves the right direction from the unit
///    of the forward direction and the view up crossing. Finally by crossing
///    the forward and the right direction, it gets the up direction.
///
/// 3. Viewing Frustum: it matches the viewport's width and height to the right
///    and up direction coordinate system respectively through the focal
///    distance. Then it gets the bottom-leftmost point from the camera's
///    origin, its horizontal and vertical match to the viewport's dimensions,
///    its focal distance and forward direction. And finally it gets the lens
///    radius from the given aperture.
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
    // Viewport Dimensions
    let theta = degrees_to_radians(vfov);
    let half_height = f64::tan(theta / 2.);
    let viewport_height = half_height * 2.;
    let viewport_width = viewport_height * ASPECT_RATIO;

    // Coordinate System basic Vectors
    let forward = (origin - target).unit();
    let right = vup.cross(forward).unit();
    let up = forward.cross(right);

    // Viewing Frustum
    let horizontal = focus_dist * viewport_width * right;
    let vertical = focus_dist * viewport_height * up;
    let bottom_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * forward;
    let lens_radius = aperture / 2.;

    (
        origin,
        horizontal,
        vertical,
        bottom_left_corner,
        right,
        up,
        lens_radius,
    )
}
