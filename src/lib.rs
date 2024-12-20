mod geometry;
mod optics;
mod renderer;

pub use {
    geometry::{
        Color,
        Direction,
        FlatPlane,
        Object,
        Position,
        Sphere,
        Vector,
    },
    optics::Camera,
    renderer::{
        Image,
        Scene,
    },
};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGTH: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
