use {
    crate::{
        common::{
            random_double,
            MAX_DEPTH,
            SAMPLES_PER_PXL,
        },
        optics::Light,
        Camera,
        Color,
        Image,
        Object,
        IMAGE_HEIGTH as height,
        IMAGE_WIDTH as width,
    },
    std::io::Result,
};

pub struct Scene {
    id:      u8,
    camera:  Camera,
    lights:  Vec<Light>,
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new(
        id: u8,
        camera: Camera,
        lights: Vec<Light>,
        mut objects: Vec<Box<dyn Object>>,
    ) -> Self {
        objects.sort_by_key(|object| -object.depth());

        Self {
            id,
            camera,
            lights,
            objects,
        }
    }

    pub fn display(&mut self) -> Result<()> {
        let mut img = Image::new(
            width as usize,
            height as usize,
        );

        for row in 0..height {
            for col in 0..width {
                let mut pxl_color = Color::default();

                for _ in 0..SAMPLES_PER_PXL {
                    let u = (col as f64 + random_double()) / (width as f64 - 1.0);
                    let v = (row as f64 + random_double()) / (height as f64 - 1.0);

                    let ray = self.camera.get_ray(u, v);
                    pxl_color += ray.color(
                        &self.objects,
                        &self.lights,
                        MAX_DEPTH,
                    );
                }

                img.set_pxl_color(
                    row as usize,
                    col as usize,
                    pxl_color,
                );
            }
        }

        img.write_ppm(
            format!(
                "assets/scenes/00{}.ppm",
                self.id
            )
            .as_str(),
        )
    }
}
