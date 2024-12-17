#[allow(unused)]
pub struct Position {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) z: usize,
}

#[allow(unused)]
impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
