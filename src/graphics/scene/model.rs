use {
    super::builder::SceneBuilder,
    crate::{
        common::{random_double, MAX_DEPTH, SAMPLES_PER_PX},
        Camera, Color, Image, Object, Result, IMAGE_HEIGTH as height,
        IMAGE_WIDTH as width,
    },
    indicatif::{ProgressBar, ProgressStyle},
    std::io::{Error, ErrorKind},
};

pub struct Scene {
    id: u8,
    camera: Camera,
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub(super) fn new(
        id: u8,
        camera: Camera,
        objects: Vec<Box<dyn Object>>,
    ) -> Self {
        Self {
            id,
            camera,
            objects,
        }
    }

    pub fn builder() -> SceneBuilder {
        SceneBuilder::default()
    }

    pub fn display(&mut self) -> Result<()> {
        let mut img = Image::new(width as usize, height as usize)?;

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

        for row in 0..height {
            for col in 0..width {
                let mut px_color = Color::default();

                for _ in 0..SAMPLES_PER_PX {
                    let u = (col as f64 + random_double())
                        / (width as f64 - 1.0);
                    let v = (row as f64 + random_double())
                        / (height as f64 - 1.0);
                    let ray = self.camera.get_ray(u, v);
                    px_color +=
                        ray.color(&self.objects, MAX_DEPTH);
                }

                img.set_px_color(row as usize, col as usize, px_color);

                progress_bar.inc(1);
            }
        }

        progress_bar
            .finish_with_message(format!("Scene {} generated", self.id));

        img.write_ppm(format!(
            "assets/scenes/00{}.ppm",
            self.id
        ))
    }
}
