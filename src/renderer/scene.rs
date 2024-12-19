use crate::{Camera, Hittable, Image, IMAGE_HEIGTH as height, IMAGE_WIDTH as width};

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            camera,
            objects,
        }
    }

    pub fn display(&self) {
        let mut img = Image::new(width as usize, height as usize);

        for row in (0..height).rev() {
            for col in 0..width {
                let u = (col / (width - 1)) as f32;
                let v = (row / (height - 1)) as f32;

                let mut ray = self.camera.get_ray(u, v);
                img.set_pxl_color(row as usize, col as usize, ray.color());
            }
        }

        img.write_ppm();
    }
}
