use rt::{Color, Scene};
use std::fs::File;
use std::io::Read;

#[test]
fn test_scene() {
    let mut img = Scene::new(3, 3);
    let filename = "test_scene.ppm";

    img.set_pixel(0, 0, Color::white());
    img.set_pixel(1, 1, Color::white());
    img.set_pixel(2, 2, Color::white());

    img.save(filename)
        .expect("Failed to save file...\n");

    let mut file = File::open(format!("./scenes/{}", filename)).expect("Failed to open file...\n");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file content...\n");

    assert_eq!(
        content,
        r#"P3
3 3
255
255 255 255
0 0 0
0 0 0
0 0 0
255 255 255
0 0 0
0 0 0
0 0 0
255 255 255
"#
    );
}
