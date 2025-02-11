mod constants;
mod errors;
mod functions;

pub(super) use {
    constants::{
        ASPECT_RATIO,
        MAX_DEPTH,
        RAYS_PER_PX,
    },
    errors::Error,
    functions::{
        clamp,
        compute,
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
    functions::init,
};
