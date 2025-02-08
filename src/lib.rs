mod camera;
mod geometry;
mod graphics;
mod material;
mod optics;
pub(self) mod utils;

pub use {
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
        Vector,
    },
    graphics::Scene,
    material::{
        Dielectric,
        Emissive,
        Lambertian,
        Metal,
    },
    std::io,
    utils::{
        get_scene_id,
        Result,
        IMAGE_HEIGTH,
        IMAGE_WIDTH,
    },
};
