mod common;
mod geometry;
mod optics;
mod renderer;

pub use {
    common::{
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
    geometry::{
        Color,
        Cylinder,
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
