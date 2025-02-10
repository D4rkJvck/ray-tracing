use {
    super::Material,
    crate::{
        optics::{
            Impact,
            Ray,
        },
        Color,
        Direction,
    },
};

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        impact: &Impact,
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction =
            impact.surface_normal() + Direction::random_unit();

        if scatter_direction.near_0() {
            scatter_direction = impact.surface_normal()
        }

        let scattered = Ray::new(impact.point(), scatter_direction);

        Some((self.albedo, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}
