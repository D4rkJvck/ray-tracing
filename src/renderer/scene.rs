use crate::{
    common::{
        random_double,
        SAMPLES_PER_PXL,
    },
    Camera,
    Color,
    Image,
    Object,
    IMAGE_HEIGTH as height,
    IMAGE_WIDTH as width,
};

pub struct Scene {
    camera:  Camera,
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new(camera: Camera, mut objects: Vec<Box<dyn Object>>) -> Self {
        objects.sort_by_key(|object| -object.depth());

        Self {
            camera,
            objects,
        }
    }

    pub fn display(&mut self) {
        let mut img = Image::new(
            width as usize,
            height as usize,
        );

        for row in 0..height {
            for col in 0..width {
                let mut pxl_color = Color::default();

                for _ in 0..SAMPLES_PER_PXL {
                    let u = (col as f64
                        + random_double()) / (width as f64 - 1.0);
                    let v = (row as f64
                        + random_double()) / (height as f64 - 1.0);

                    let ray = self.camera.get_ray(u, v);
                    pxl_color += ray.color(&self.objects);
                }

                img.set_pxl_color(
                    row as usize,
                    col as usize,
                    pxl_color,
                );
            }
        }

        img.write_ppm();
    }
}
