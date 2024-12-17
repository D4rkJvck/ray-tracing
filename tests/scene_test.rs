use rt::{Color, Scene};
use std::fs::File;
use std::io::Read;

#[test]
fn test_scene() {
    let width = 4;
    let height = 4;
    let mut img = Scene::new(width, height);
    let filename = "test_scene.ppm";

    for row in 0..height {
        for col in 0..width {
            if row == col {
                img.set_pixel(row, col, Color::white());
            }
        }
    }

    img.save(filename)
        .expect("Failed to save file...\n");

    let mut file = File::open(format!("./scenes/{}", filename)).expect("Failed to open file...\n");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file content...\n");

    assert_eq!(
        content,
        r#"P3
4 4
255
255 255 255
0 0 0
0 0 0
0 0 0
0 0 0
255 255 255
0 0 0
0 0 0
0 0 0
0 0 0
255 255 255
0 0 0
0 0 0
0 0 0
0 0 0
255 255 255
"#
    );
}
