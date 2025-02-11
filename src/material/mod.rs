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
    fn scatter(&self, _ray: &Ray, _impact: &Impact) -> Option<(Color, Ray)> { None }
    //--------------------------------------------------------
    fn reflectance(cosine: f64, ref_idx: f64) -> f64
    where
        Self: Sized,
    {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * f64::powf(1. - cosine, 5.)
    }
    //--------------------------------------------------------
    fn emit(&self) -> Color { Color::default() }
}
