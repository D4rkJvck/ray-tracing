use {
    super::builder::SceneBuilder,
    crate::{
        utils::{
            random_double,
            MAX_DEPTH,
            SAMPLES_PER_PX,
        },
        Camera,
        Color,
        Image,
        Object,
        Result,
        IMAGE_HEIGTH as height,
        IMAGE_WIDTH as width,
    },
    indicatif::{
        ProgressBar,
        ProgressStyle,
    },
    rayon::prelude::*,
    std::io::{
        Error,
        ErrorKind,
    },
};

pub struct Scene {
    id:      u8,
    camera:  Camera,
    objects: Vec<Box<dyn Object>>,
    image:   Image,
}

impl Scene {
    pub(super) fn new(
        id: u8,
        camera: Camera,
        objects: Vec<Box<dyn Object>>,
        image: Image,
    ) -> Self {
        Self {
            id,
            camera,
            objects,
            image,
        }
    }

    pub fn builder() -> SceneBuilder { SceneBuilder::default() }

    pub fn display(&mut self) -> Result<()> {
        let total_px = (width * height) as u64;
        let progress_bar = ProgressBar::new(total_px);
        let style_tmpl = format!(
            "[{{elapsed_precise}}] |{{bar:50.green/black}}| \
             [{{pos}}/{}px] [{{eta}}]",
            total_px
        );
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(style_tmpl.as_str())
                .map_err(|error| Error::new(ErrorKind::Other, error))?
                .progress_chars("=|-"),
        );

        // Create a vector of all pixel coordinates
        let pixels: Vec<(usize, usize)> = (0..height as usize)
            .flat_map(|row| (0..width as usize).map(move |col| (row, col)))
            .collect();

        // Process pixels in parallel
        let colors: Vec<(usize, usize, Color)> = pixels
            .par_iter()
            .map(|(row, col)| {
                let mut px_color = Color::default();

                for _ in 0..SAMPLES_PER_PX {
                    let u = (*col as f64 + random_double())
                        / (width as f64 - 1.0);
                    let v = (*row as f64 + random_double())
                        / (height as f64 - 1.0);
                    let ray = self.camera.get_ray(u, v);
                    px_color += ray.color(&self.objects, MAX_DEPTH);
                }

                progress_bar.inc(1);
                (*row, *col, px_color)
            })
            .collect();

        // Write the results to the image
        for (row, col, color) in colors {
            self.image
                .set_px_color(row, col, color);
        }

        progress_bar
            .finish_with_message(format!("Scene {} generated", self.id));

        self.image.write_ppm(format!(
            "assets/scenes/00{}.ppm",
            self.id
        ))?;

        Ok(())
    }
}
