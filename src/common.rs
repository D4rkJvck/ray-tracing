use {
    rand::Rng,
    std::f64::consts::PI,
};

const ASPECT_RATIO: f64 = 4.0 / 3.0;
pub const IMAGE_WIDTH: i32 = 800;
pub const IMAGE_HEIGTH: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
pub const SAMPLES_PER_PXL: i32 = 1;

pub fn degrees_to_radians(degrees: f64) -> f64 { degrees * PI / 180.0 }

pub fn random_double() -> f64 { rand::thread_rng().gen() }

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
