mod color;
mod position;
mod direction;

pub use color::Color;
pub use position::Position;
pub use direction::Direction;

#[allow(unused)]
pub(self) trait Vector {}
