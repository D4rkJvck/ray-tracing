mod common;
mod geometry;
mod graphics;
pub mod optics; // Make optics module public

pub use {
    common::{
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
    geometry::{
        Color,
        // Cube,
        // Cylinder,
        Direction,
        Plane,
        Object,
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
