mod common;
mod geometry;
pub mod optics; // Make optics module public
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
    optics::{
        Camera,
        Light,
    },
    renderer::{
        Image,
        Scene,
    },
};
