use {
    super::Material,
    crate::{
        optics::Ray,
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
        _ray: &crate::optics::Ray,
        impact: &crate::optics::Impact,
    ) -> Option<(Color, Ray)> {
        let scatter_direction =
            impact.surface_normal + Direction::random_unit();
        let scattered = Ray::new(impact.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}
