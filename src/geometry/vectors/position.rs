#[allow(unused)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

#[allow(unused)]
impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
