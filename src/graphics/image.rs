use {
    crate::{
        common::Error,
        Color,
        Result,
    },
    std::io::Write,
};

pub struct Image {
    width:     usize,
    height:    usize,
    px_colors: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::InvalidDimension(
                "Dimensions must be greater than 0",
            ));
        }

        let px_colors = vec![vec![Color::default(); width]; height];

        Ok(Self {
            width,
            height,
            px_colors,
        })
    }

    /// This function will simply set the `color` of the given pixel
    /// through its position's coordonates in the vector of pixels.
    pub fn set_px_color(&mut self, row: usize, col: usize, color: Color) {
        self.px_colors[row][col] = color;
    }

    /// This function is responsible for generating the image file
    /// in `.ppm` format in the `scenes/` directory from the renderer's
    /// implementation's result.
    pub fn write_ppm(&self, output_file: String) -> Result<()> {
        let mut file = std::fs::File::create(output_file)?;
        writeln!(
            &mut file,
            "P3\n{} {}\n255",
            self.width, self.height
        )?;

        for color in self
            .px_colors
            .iter()
            .rev()
            .flat_map(|row| row.iter())
        {
            writeln!(&mut file, "{color}")?
        }

        Ok(())
    }
}
