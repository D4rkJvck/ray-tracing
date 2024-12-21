use crate::{
    Camera,
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
    pub fn new(camera: Camera, objects: Vec<Box<dyn Object>>) -> Self {
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
                let u = col as f32 / (width as f32 - 1.0);
                let v = row as f32 / (height as f32 - 1.0);

                let ray = self.camera.get_ray(u, v);
                let color = ray.color(&mut self.objects);

                img.set_pxl_color(
                    row as usize,
                    col as usize,
                    color,
                );
            }
        }

        img.write_ppm();
    }
}
