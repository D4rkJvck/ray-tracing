use rt::{
    Camera,
    Color,
    // Cube,
    // Cylinder,
    Direction,
    Light,
    Plane,
    Position,
    Scene,
    Sphere,
};

fn main() {
    // Scène 1 : Une sphère
    let mut scene1 = Scene::new(
        1,
        Camera::new(
            Position::new(-2., 2., 1.),
            Position::new(0., 0., -1.),
            Direction::new(0., 1., 0.),
            90.,
        ),
        vec![Light::new(
            Position::new(2.0, 2.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
            1.0,
        )],
        vec![
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -1.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),
            Box::new(Plane::new(
                Position::new(0.0, -0.5, -1.0),
                Position::new(0.0, 1.0, 0.0),
                Color::new(0.5, 0.5, 0.5),
            )),
        ],
    );

    if let Err(error) = scene1.display() {
        dbg!(error);
    }

    // Scène 2 : Plan et cube avec luminosité réduite
    // let mut scene2 = Scene::new(
    //     2,
    //     Camera::new(Position::default()),
    //     vec![Light::new(
    //         Position::new(2.0, 2.0, -1.0),
    //         Color::new(1.0, 1.0, 1.0),
    //         0.5, // Luminosité réduite
    //     )],
    //     vec![
    //         Box::new(Plane::new(
    //             Position::new(0.0, -0.5, -1.0),
    //             Position::new(0.0, 1.0, 0.0),
    //             Color::new(0.5, 0.5, 0.5),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(0.0, 0.0, -1.0),
    //             1.0,
    //             Color::new(0.0, 0.2, 0.5),
    //         )),
    //     ],
    // );

    // scene2.display();

    // // Scène 3 : Tous les objets
    // let mut scene3 = Scene::new(
    //     3,
    //     Camera::new(Position::new(0.0, 1.0, 0.0)),
    //     vec![Light::new(
    //         Position::new(-2.0, 2.0, -1.0),
    //         Color::new(1.0, 1.0, 1.0),
    //         1.0,
    //     )],
    //     vec![
    //         Box::new(Sphere::new(
    //             Position::new(0.0, 0.0, -2.0),
    //             0.5,
    //             Color::new(0.1, 0.0, 0.0),
    //         )),
    //         Box::new(Cylinder::new(
    //             Position::new(2.0, -0.5, -2.0),
    //             0.5,
    //             1.6,
    //             Direction::new(0.0, 1.0, 0.0),
    //             Color::new(0.2, 0.2, 0.2),
    //         )),
    //         Box::new(Plane::new(
    //             Position::new(0.0, -0.5, -1.0),
    //             Position::new(0.0, 1.0, 0.0),
    //             Color::new(0.1, 0.1, 0.0),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(-1.5, 0.0, -2.5),
    //             1.0,
    //             Color::new(0.0, 0.2, 0.5),
    //         )),
    //     ],
    // );
    // scene3.display();

    // let mut scene4 = Scene::new(
    //     4,
    //     Camera::new(Position::new(2.0, 1.0, 1.0)), // Caméra déplacée
    //     vec![Light::new(
    //         Position::new(2.0, 2.0, -1.0),
    //         Color::new(1.0, 1.0, 1.0),
    //         1.0,
    //     )],
    //     vec![
    //         Box::new(Sphere::new(
    //             Position::new(0.0, 0.0, -2.0),
    //             0.5,
    //             Color::new(0.1, 0.0, 0.0),
    //         )),
    //         Box::new(Cylinder::new(
    //             Position::new(2.0, -0.5, -3.0),
    //             0.5,
    //             1.6,
    //             Direction::new(0.0, 1.0, 0.0),
    //             Color::new(0.2, 0.2, 0.2),
    //         )),
    //         Box::new(Plane::new(
    //             Position::new(0.0, -0.5, -1.0),
    //             Position::new(0.0, 1.0, 0.0),
    //             Color::new(0.1, 0.1, 0.0),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(-1.5, 0.0, -2.5),
    //             1.0,
    //             Color::new(0.0, 0.2, 0.5),
    //         )),
    //     ],
    // );
    // scene4.display();
}
