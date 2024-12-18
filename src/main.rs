// use rt::*;

use rt::{Color, Scene};

fn main() {
    let width = 4;
    let height = 4;
    let mut img = Scene::new(width, height);

    for row in 0..height {
        for col in 0..width {
            if row == col {
                img.set_pixel(
                    row,
                    col,
                    Color::new(255, 255, 255),
                );
            }
        }
    }

    img.display();
}
