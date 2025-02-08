mod camera;
mod geometry;
mod graphics;
mod material;
mod optics;
mod utils;

pub(self) use utils::{
    compute,
    validate_params,
};
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
    graphics::{
        Image,
        Scene,
    },
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
