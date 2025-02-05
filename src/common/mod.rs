mod constants;
mod error;
mod utils;

pub use {
    constants::{
        ASPECT_RATIO, IMAGE_HEIGTH, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PX,
    },
    error::{Error, Result},
    utils::{
        clamp, degrees_to_radians, random_double, random_double_range,
    },
};
