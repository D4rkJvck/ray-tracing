use rt::{Camera, Image, IMAGE_HEIGTH as height, IMAGE_WIDTH as width};

fn main() {
    let camera = Camera::new(2.0, 1.0);
    let mut img = Image::new(width, height);

    for row in 0..height {
        for col in 0..width {
            let u = (col / (width - 1)) as f32;
            let v = (row / (height - 1)) as f32;

            let ray = camera.get_ray(u, v);
            img.set_pxl_color(row, col, ray.color());
        }
    }

    img.display();
}
