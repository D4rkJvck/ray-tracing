mod cube;
mod cylinder;
mod plane;
mod sphere;

use {
    super::Position,
    crate::{
        material::Material,
        optics::{
            Impact,
            Ray,
        },
    },
};
pub use {
    cube::Cube,
    cylinder::Cylinder,
    plane::Plane,
    sphere::Sphere,
};

pub trait Object: Send + Sync {
    fn material(&self) -> &dyn Material;
    fn position(&self) -> Position;
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact>;
    fn depth(&self, position: Position) -> i32 {
        ((self.position() - position).length() * 1e6) as i32
    }
    // fn light_contribution(&self, ray: &Ray) -> Color {
    //     let emission = self.material().emit();
    //     if emission == Color::default() {
    //         None
    //     }
    //     else {
    //         let dir_to_light = (self.position() - ray.origin()).unit();
    //         let light_factor = ray
    //             .direction()
    //             .dot(dir_to_light)
    //             .max(0.);
    //         Some(emission * light_factor * 0.1)
    //     }
    // }
}
