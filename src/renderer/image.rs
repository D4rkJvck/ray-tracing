use crate::Color;
use std::io::Write;

pub struct Image {
    width:      usize,
    height:     usize,
    pxl_colors: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pxl_colors = vec![vec![Color::default(); width]; height];

        Self {
            width,
            height,
            pxl_colors,
        }
    }

    /// This function will simply set the `color` of the given pixel
    /// through its position's coordonates in the vector of pixels.

    pub fn set_pxl_color(&mut self, row: usize, col: usize, color: Color) {
        self.pxl_colors[row][col] = color;
    }

    /// This function is responsible for generating the image file
    /// in `.ppm` format in the `scenes/` directory from the renderer's
    /// implementation's result.
    pub fn write_ppm(&self, output_file: &str) {
        let mut file = std::fs::File::create(output_file).unwrap();
        writeln!(&mut file, "P3\n{} {}\n255", self.width, self.height).unwrap();

        self.pxl_colors
            .iter()
            .rev()
            .flat_map(|row| row.iter())
            .for_each(|color| writeln!(&mut file, "{color}").unwrap());
    }
  
}
