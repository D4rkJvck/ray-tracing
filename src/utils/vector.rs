#[allow(unused)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[allow(unused)]
impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
