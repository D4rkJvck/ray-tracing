mod common;
mod error;
mod geometry;
mod graphics;
mod optics;
mod utils;

pub use {
    common::{
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
    error::Result,
    geometry::{
        Color,
        // Cube,
        Cylinder,
        Direction,
        Object,
        Plane,
        Position,
        Sphere,
        Vector,
    },
    graphics::{
        Image,
        Scene,
    },
    optics::{
        Camera,
        Light,
    },
};
