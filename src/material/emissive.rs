use {
    super::Material,
    crate::Color,
};

pub struct Emissive {
    albedo:    Color,
    intensity: f64,
}

impl Material for Emissive {
    fn emit(&self) -> Color { self.albedo * self.intensity }
}

impl Emissive {
    pub fn new(albedo: Color, intensity: f64) -> Self {
        Self {
            albedo,
            intensity,
        }
    }
}
