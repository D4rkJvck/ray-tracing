mod lambertian;
mod metal;

pub use {lambertian::Lambertian, metal::Metal};

use crate::{
    optics::{Impact, Ray},
    Color,
};

pub trait Material {
    fn scatter(&self, ray: &Ray, impact: &Impact) -> Option<(Color, Ray)>;
    fn emit(&self) -> Color {
        Color::default()
    }
}
