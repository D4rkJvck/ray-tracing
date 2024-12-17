use super::position::Position;

#[allow(unused)]
pub struct Direction {
    dx: usize,
    dy: usize,
    dz: usize,
}

#[allow(unused)]
impl Direction {
    pub fn new(origin: Position, target: Position) -> Self {
        Self {
            dx: target.x - origin.x,
            dy: target.y - origin.y,
            dz: target.z - origin.z,
        }
    }
}
