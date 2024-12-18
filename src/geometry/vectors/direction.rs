use super::position::Position;

#[allow(unused)]
pub struct Direction {
    dx: f64,
    dy: f64,
    dz: f64,
}

#[allow(unused)]
impl Direction {
    pub fn new(origin: Position, target: Position) -> Self {
        Self {
            dx: target.x() - origin.x(),
            dy: target.y() - origin.y(),
            dz: target.z() - origin.z(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.dx.powf(2.0)
            + self.dy.powf(2.0)
            + self.dz.powf(2.0))
        .sqrt()
    }

    pub fn normalize(&self) {}
}
