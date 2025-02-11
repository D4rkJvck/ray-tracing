use {
    super::Scene,
    crate::{
        graphics::Image,
        utils::{
            random_double,
            MAX_DEPTH,
            RAYS_PER_PX,
        },
        Result,
        IMAGE_HEIGTH as height,
        IMAGE_WIDTH as width,
    },
    indicatif::{
        ProgressBar,
        ProgressStyle,
    },
    rayon::prelude::*,
    std::sync::Mutex,
};

//------------------------------------------------------------------------------------------------------------------------------------------------

impl Scene {
    pub fn draw(&mut self) -> Result<()> {
        let mut image = Image::new(width as usize, height as usize)?;
        let image_guard = Mutex::new(&mut image);

        let total_px = (width * height) as u64;
        let progress_bar = ProgressBar::new(total_px);
        let style_tmpl = format!(
            "\n[{{elapsed_precise}}] |{{bar:50.green/black}}| [{{pos}}/{}px] [{{eta}}]\n{{msg}}",
            total_px
        );
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(style_tmpl.as_str())?
                .progress_chars("=|-"),
        );

        // Create a vector of all pixel coordinates
        (0..height)
            .into_par_iter()
            .for_each(|row| {
                (0..width)
                    .into_par_iter()
                    .for_each(|col| {
                        (0..RAYS_PER_PX)
                            .into_par_iter()
                            .for_each(|_| {
                                let u = (col as f64 + random_double()) / (width as f64 - 1.0);
                                let v = (row as f64 + random_double()) / (height as f64 - 1.0);
                                let ray = self.camera.get_ray(u, v);
                                let color = ray.color(
                                    &self.objects,
                                    self.brightness,
                                    MAX_DEPTH,
                                );

                                let mut image_lock = image_guard.lock().unwrap();
                                image_lock.acc_color_per_px(row, col, color);
                            });

                        progress_bar.inc(1);
                    })
            });

        progress_bar.finish_with_message(format!(
            "\nScene 00{} [GENERATED]\nCreating PPM File...",
            self.id
        ));

        image.write_ppm(format!(
            "assets/scenes/00{}.ppm",
            self.id
        ))?;

        Ok(println!("PPM File [CREATED]"))
    }
}
