use rt::{Color, Scene};

#[test]
fn test_scene() {
    let width = 4;
    let height = 4;
    let mut img = Scene::new(width, height);
    let content = "";

    for row in 0..height {
        for col in 0..width {
            if row == col {
                img.set_pixel(
                    row,
                    col,
                    Color::new(255.0, 255.0, 255.0),
                );
            }
        }
    }

    img.display();


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
