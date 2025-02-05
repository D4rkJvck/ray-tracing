use {
    super::Material,
    crate::{
        optics::Ray,
        Color, Direction,
    },
};

pub struct Metal {
    albedo: Color,
    fuzz:   f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::optics::Ray,
        impact: &crate::optics::Impact,
    ) -> Option<(Color, crate::optics::Ray)> {
        let reflected = ray
            .direction()
            .unit()
            .reflect(impact.surface_normal);

        let scattered = Ray::new(impact.point, reflected + self.fuzz * Direction::random_unit());

        Some((self.albedo, scattered))
    }
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}
