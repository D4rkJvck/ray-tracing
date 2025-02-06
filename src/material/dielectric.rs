use {
    super::Material,
    crate::{
        optics::{
            Impact,
            Ray,
        },
        Color,
    },
};

pub struct Dielectric {
    refract_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, impact: &Impact) -> Option<(Color, Ray)> {
        // let refract_ratio = if impact.front_face {
        //     1. / self.refract_idx
        // }
        // else {
        //     self.refract_idx
        // };

        let unit_direction = ray.direction().unit();
        let refracted = unit_direction
            .refract(impact.surface_normal, self.refract_idx);
        let scattered = Ray::new(impact.point, refracted);

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
