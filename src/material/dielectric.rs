use {
    super::Material,
    crate::{
        optics::{
            Impact,
            Ray,
        },
        utils::random_double,
        Color,
    },
};

pub struct Dielectric {
    refract_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, impact: &Impact) -> Option<(Color, Ray)> {
        let refract_ratio = if impact.front_face() {
            1. / self.refract_idx
        }
        else {
            self.refract_idx
        };

        let unit_direction = ray.direction().unit();

        let cos_theta = -unit_direction
            .dot(impact.surface_normal())
            .min(1.);
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

        let cannot_refract = refract_ratio * sin_theta > 1.;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refract_ratio)
                > random_double()
        {
            unit_direction.reflect(impact.surface_normal())
        }
        else {
            unit_direction.refract(impact.surface_normal(), refract_ratio)
        };

        let scattered = Ray::new(impact.point(), direction);

        Some((Color::new(1., 1., 1.), scattered))
    }
}

impl Dielectric {
    pub fn new(refract_idx: f64) -> Self {
        Self {
            refract_idx,
        }
    }
}
