mod dielectric;
mod emissive;
mod lambertian;
mod metal;

pub use {
    dielectric::Dielectric,
    emissive::Emissive,
    lambertian::Lambertian,
    metal::Metal,
};

use crate::{
    optics::{
        Impact,
        Ray,
    },
    Color,
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _ray: &Ray,
        _impact: &Impact,
    ) -> Option<(Color, Ray)> {
        None
    }
    fn emit(&self) -> Color { Color::default() }
}
