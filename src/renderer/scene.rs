use crate::Color;

#[allow(unused)]
pub struct Scene {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

#[allow(unused)]
impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels =
            vec![vec![Color::default(); width]; height];

        Self {
            width,
            height,
            pixels,
        }
    }

    /// This function will simply set the `color` of the given pixel
    /// through its position's coordonates in the vector of pixels.
    pub fn set_pixel(
        &mut self,
        row: usize,
        col: usize,
        color: Color,
    ) {
        self.pixels[row][col] = color
    }

    /// This function is responsible for generating the image file
    /// in `.ppm` format in the `scenes/` directory from the renderer's
    /// implementation's result.
    pub fn display(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for row in self.pixels.iter() {
            for color in row {
                println!("{color}")
            }
        }
    }
}
