use std::{error::Error, fs, io::Write, path::Path};

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
        let pixels = vec![vec![Color::default(); width]; height];

        Self {
            width,
            height,
            pixels,
        }
    }

    /// This function will simply set the `color` of the given pixel
    /// through its position's coordonates in the vector of pixels.
    pub fn set_pixel(&mut self, row: usize, col: usize, color: Color) {
        self.pixels[row][col] = color
    }

    /// This function is responsible for generating the image file
    /// in `.ppm` format in the `scenes/` directory from the renderer's
    /// implementation's result.
    pub fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let dir = "scenes";
        fs::create_dir_all(dir)?;

        let file_path = Path::new(dir).join(filename);
        let mut file = fs::File::create(file_path)?;

        file.write(b"P3\n")?;
        file.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        file.write(b"255\n")?;

        for row in self.pixels.iter() {
            for color in row {
                writeln!(file, "{color}");
            }
        }

        Ok(())
    }
}
