use rt::{
    get_scene_id,
    Camera,
    Color,
    Cube,
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
    IMAGE_HEIGTH as height,
    IMAGE_WIDTH as width,
};

fn main() -> Result<()> {
    let id = get_scene_id()?;

    let camera = Camera::builder()
        .origin(Position::new(0., 4., 0.))
        .target(Position::new(0., 0., -8.))
        .view_up(Position::new(0., 1., 0.))
        .vertical_field_of_view(60.)
        .aperture(0.1)
        .build()?;

    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(
            Position::new(0., 240., -240.),
            100.,
            Box::new(Emissive::new(
                Color::new(1., 1., 1.),
                15.,
            )),
        )),
        Box::new(Sphere::new(
            Position::new(0., 1., -8.),
            1.,
            Box::new(Metal::new(
                Color::new(0.2, 0.2, 0.2),
                0.,
            )),
        )),
        Box::new(Cylinder::new(
            Position::new(-3., 0., -8.),
            0.75,
            1.5,
            Direction::new(0., 1., 0.),
            Box::new(Metal::new(Color::new(0.5, 0., 0.), 1.)),
        )),
        Box::new(Cube::new(
            Position::new(3., 0.75, -8.),
            1.5,
            Box::new(Lambertian::new(Color::new(0., 0., 1.))),
        )),
        Box::new(Plane::new(
            Position::new(0., 0., -120.),
            Position::new(0., 1., 0.),
            Box::new(Lambertian::new(Color::new(1., 0.5, 0.))),
        )),
    ];

    Scene::builder()
        .id(id)
        .camera(camera)
        .objects(objects)
        .image_size(width, height)
        .build()?
        .display()
}
