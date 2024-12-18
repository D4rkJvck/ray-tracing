#[allow(unused)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(unused)]
impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(super) fn x(&self) -> f64 {
        self.x
    }

    pub(super) fn y(&self) -> f64 {
        self.y
    }

    pub(super) fn z(&self) -> f64 {
        self.z
    }
}
