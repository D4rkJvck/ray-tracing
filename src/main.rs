use rt::{
    Camera,
    Color,
    FlatPlane,
    Position,
    Scene,
    Sphere,
};

fn main() {
    let camera = Camera::new(
        Position::new(0.0, 0.0, 0.0),
        2.0,
        1.0,
    );

    let scene_for_plane = Scene::new(
        camera,
        vec![
            Box::new(FlatPlane::new(
                Position::new(0.0, -0.5, -2.0),
                Position::new(0.0, 1.0, 0.0),
                Color::new(0.0, 1.0, 0.0),
            )),
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -1.0),
                0.5,
                Color::new(1.0, 0.0, 0.0),
            )),
        ],
    );

    scene_for_plane.display();
}
