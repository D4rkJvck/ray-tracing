use crate::{
    Camera,
    Image,
    Object,
    IMAGE_HEIGTH as height,
    IMAGE_WIDTH as width,
};

#[allow(unused)]
pub struct Scene<T: Object> {
    camera:  Camera,
    objects: Vec<T>,
}

impl<T: Object> Scene<T> {
    pub fn new(camera: Camera, objects: Vec<T>) -> Self {
        Self { camera,
               objects }
    }

    pub fn display(&self) {
        let mut img = Image::new(width as usize,
                                 height as usize);

        for row in 0..height {
            for col in 0..width {
                let u = col as f32 / (width as f32 - 1.0);
                let v = row as f32 / (height as f32 - 1.0);

                let ray = self.camera.get_ray(u, v);
                let color = ray.color(&self.objects);

                img.set_pxl_color(row as usize,
                                  col as usize,
                                  color);
            }
        }

        img.write_ppm();
    }
}
