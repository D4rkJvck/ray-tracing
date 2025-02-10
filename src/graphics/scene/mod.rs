mod draw;
mod generate;

use crate::{
    Camera,
    Object,
};

pub struct Scene {
    id:      u8,
    camera:  Camera,
    objects: Vec<Box<dyn Object>>,
}
