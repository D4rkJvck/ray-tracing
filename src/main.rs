use rt::{
    Camera,
    Color,
    Cylinder,
    Dielectric,
    Direction,
    Emissive,
    Lambertian,
    Metal,
    Object,
    Plane,
    Position,
    Result,
    Scene,
    Sphere,
};

fn main() -> Result<()> {
    let camera = Camera::builder()
        .origin(Position::new(1., 2., 3.))
        .target(Position::new(0., 0., -1.))
        .view_up(Position::new(0., 0.1, 0.))
        .vertical_field_of_view(90.)
        .aperture(0.1)
        .build()?;

    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(
            Position::new(0., 1., -3.),
            0.5,
            Box::new(Emissive::new(
                Color::new(1., 1., 1.),
                25.,
            )),
        )),
        Box::new(Cylinder::new(
            Position::new(-1., -0.5, -2.),
            0.5,
            1.5,
            Direction::new(0., 1., 0.),
            Box::new(Metal::new(Color::new(0.5, 0., 0.), 0.)),
        )),
        Box::new(Plane::new(
            Position::new(2., -0.5, -100.),
            Position::new(0., 1., 0.),
            Box::new(Lambertian::new(Color::new(
                0.1, 0.05, 0.,
            ))),
        )),
    ];

    Scene::builder()
        .id(1)
        .camera(camera)
        .add_objects(objects)
        .build()?
        .display()

    // Scène 2 : Plan et cube avec luminosité réduite
    // let mut scene2 = Scene::new(
    //     2,
    //     Camera::new(Position::default()),
    //     vec![Light::new(
    //         Position::new(2., 2., -1.),
    //         Color::new(1., 1., 1.),
    //         0.5, // Luminosité réduite
    //     )],
    //     vec![
    //         Box::new(Plane::new(
    //             Position::new(0., -0.5, -1.),
    //             Position::new(0., 1., 0.),
    //             Color::new(0.5, 0.5, 0.5),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(0., 0., -1.),
    //             1.,
    //             Color::new(0., 0.2, 0.5),
    //         )),
    //     ],
    // );

    // scene2.display();

    // // Scène 3 : Tous les objets
    // let mut scene3 = Scene::new(
    //     3,
    //     Camera::new(Position::new(0., 1., 0.)),
    //     vec![Light::new(
    //         Position::new(-2., 2., -1.),
    //         Color::new(1., 1., 1.),
    //         1.,
    //     )],
    //     vec![
    //         Box::new(Sphere::new(
    //             Position::new(0., 0., -2.),
    //             0.5,
    //             Color::new(0.1, 0., 0.),
    //         )),
    //         Box::new(Cylinder::new(
    //             Position::new(2., -0.5, -2.),
    //             0.5,
    //             1.6,
    //             Direction::new(0., 1., 0.),
    //             Color::new(0.2, 0.2, 0.2),
    //         )),
    //         Box::new(Plane::new(
    //             Position::new(0., -0.5, -1.),
    //             Position::new(0., 1., 0.),
    //             Color::new(0.1, 0.1, 0.),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(-1.5, 0., -2.5),
    //             1.,
    //             Color::new(0., 0.2, 0.5),
    //         )),
    //     ],
    // );
    // scene3.display();

    // let mut scene4 = Scene::new(
    //     4,
    //     Camera::new(Position::new(2., 1., 1.)), // Caméra déplacée
    //     vec![Light::new(
    //         Position::new(2., 2., -1.),
    //         Color::new(1., 1., 1.),
    //         1.,
    //     )],
    //     vec![
    //         Box::new(Sphere::new(
    //             Position::new(0., 0., -2.),
    //             0.5,
    //             Color::new(0.1, 0., 0.),
    //         )),
    //         Box::new(Cylinder::new(
    //             Position::new(2., -0.5, -3.),
    //             0.5,
    //             1.6,
    //             Direction::new(0., 1., 0.),
    //             Color::new(0.2, 0.2, 0.2),
    //         )),
    //         Box::new(Plane::new(
    //             Position::new(0., -0.5, -1.),
    //             Position::new(0., 1., 0.),
    //             Color::new(0.1, 0.1, 0.),
    //         )),
    //         Box::new(Cube::new(
    //             Position::new(-1.5, 0., -2.5),
    //             1.,
    //             Color::new(0., 0.2, 0.5),
    //         )),
    //     ],
    // );
    // scene4.display();
}
