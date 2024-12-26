use rt::{
    Camera,
    Color,
    Light,  // Import Light directly
    Position,
    Scene,
    Sphere,
    // Cylinder,
    // FlatPlane,
    // Direction,
};

fn main() {
    let mut scene = Scene::new(
        Camera::new(Position::default()),
        vec![
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -1.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),
            //  Box::new(FlatPlane::new(
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
        ],
        vec![
            Light::new(
                Position::new(2.0, 2.0, -1.0),
                Color::new(1.0, 1.0, 1.0),
                1.0
            ),
            Light::new(
                Position::new(-2.0, 1.0, -1.0),
                Color::new(0.5, 0.5, 1.0),
                0.6
            ),
        ],
    );

    scene.display();
}