#[allow(unused)]
#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(unused)]
impl Color {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn custom(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_ppm(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}
