mod camera;
mod geometry;
mod light;
mod renderer;

pub use camera::Camera;
pub use geometry::*;
pub use renderer::Image;

const ASPECT_RATIO: f32 = 4.0 / 3.0;
pub const IMAGE_WIDTH: usize = 400;
pub const IMAGE_HEIGTH: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
