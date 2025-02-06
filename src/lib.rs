mod camera;
mod common;
mod geometry;
mod graphics;
mod material;
mod optics;

pub use {
    camera::Camera,
    common::{Result, IMAGE_HEIGTH, IMAGE_WIDTH},
    geometry::{
        Color, Cube, Cylinder, Direction, Object, Plane, Position, Sphere,
        Vector,
    },
    graphics::{
        Image,
        Scene,
    },
    material::{
        Lambertian,
        Metal,
        Dielectric,
        Emissive,
    },
};
