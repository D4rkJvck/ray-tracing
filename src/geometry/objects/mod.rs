mod cube;
mod cylinder;
mod plane;
mod sphere;

use crate::optics::Ray;

pub trait Object {
    fn get_hit(&self, ray: Ray) -> bool;
}
