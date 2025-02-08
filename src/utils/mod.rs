mod constants;
mod errors;
mod functions;

pub(crate) use {
    constants::{
        ASPECT_RATIO,
        MAX_DEPTH,
        SAMPLES_PER_PX,
    },
    errors::Error,
    functions::{
        clamp,
        compute,
        degrees_to_radians,
        random_double,
        random_double_range,
        validate_params,
    },
};
pub use {
    constants::{
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
    errors::Result,
    functions::get_scene_id,
};
