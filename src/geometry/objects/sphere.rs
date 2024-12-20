use {
    super::Object,
    crate::{optics::Ray, Position},
};

#[allow(unused)]
pub struct Sphere {
    center: Position,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Position, radius: f32) -> Self {
        Self { center,
               radius }
    }
}

impl Object for Sphere {
    fn get_hit(&self, ray: Ray) -> bool {
        todo!()
    }
}
