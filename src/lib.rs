mod geometry;
mod optics;
mod renderer;

pub use geometry::*;
pub use optics::Camera;
pub use renderer::Image;
pub use renderer::Scene;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGTH: i32 =
    (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
