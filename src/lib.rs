mod camera;
mod geometry;
mod graphics;
mod material;
mod optics;
pub(self) mod utils;

pub(self) use {
    camera::Camera,
    geometry::{
        Color,
        Cube,
        Cylinder,
        Direction,
        Object,
        Plane,
        Position,
        Sphere,
    },
    material::{
        Dielectric,
        Emissive,
        Lambertian,
        Metal,
    },
    utils::{
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
};
pub use {
    geometry::Vector,
    graphics::Scene,
    utils::{
        init,
        Result,
    },
};
