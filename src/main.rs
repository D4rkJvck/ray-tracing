use rt::{
    Camera,
    Color,
    Cylinder,
    Direction,
    FlatPlane,
    Position,
    Scene,
    Sphere,
};

fn main() {
    let mut scene = Scene::new(
        Camera::new(Position::new(
            0.0, 0.25, 0.0,
        )),
        vec![
            // Box::new(FlatPlane::new(
            //     Position::new(0.0, 0.0, -100.0),
            //     Position::new(0.0, 0.1, 0.0),
            //     Color::new(0.5, 0.5, 0.0),
            // )),
            // Box::new(Cylinder::new(
            //     Position::new(-1.0, 0.0, -1.0),
            //     0.2,
            //     1.0,
            //     Direction::new(1.0, 1.0, -1.0),
            //     Color::new(0.8, 0.0, 0.8),
            // )),
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -1.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),
        ],
    );

    scene.display();
}
