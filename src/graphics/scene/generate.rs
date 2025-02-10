use {
    super::Scene,
    crate::{
        utils::Error,
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
        Sphere,
    },
};

//------------------------------------------------------------------------------------------------------------------------------------------------

impl Scene {
    pub fn gen(id: u8) -> Result<Self> {
        let mut camera = Camera::builder().build()?;
        let mut objects: Vec<Box<dyn Object>> = Vec::new();

        match id {
            1 => objects.extend([
                Box::new(Sphere::new(
                    Position::new(240., 120., 0.),
                    100.,
                    Box::new(Emissive::new(
                        Color::new(1., 1., 1.),
                        15.,
                    )),
                )) as Box<dyn Object>,
                Box::new(Sphere::new(
                    Position::new(0., 1., -1.),
                    1.,
                    Box::new(Lambertian::new(Color::new(
                        0.5, 0.5, 0.5,
                    ))),
                )),
            ]),
            2 => {
                camera = Camera::builder()
                    .origin(Position::new(2., 1., 0.))
                    .target(Position::new(0., 0., -2.))
                    .vertical_field_of_view(120.)
                    .build()?;

                objects.extend([
                    Box::new(Cube::new(
                        Position::new(0., 0.75, -2.),
                        1.5,
                        Box::new(Lambertian::new(Color::new(
                            0.69, 0.57, 0.43,
                        ))),
                    )) as Box<dyn Object>,
                    Box::new(Plane::new(
                        Position::new(0., 0., -120.),
                        Position::new(0., 1., 0.),
                        Box::new(Lambertian::new(Color::new(
                            0.12, 0.39, 0.12,
                        ))),
                    )),
                ])
            }
            3 => {
                camera = Camera::builder()
                    .origin(Position::new(0., 4., 0.))
                    .target(Position::new(0., 0., -8.))
                    .vertical_field_of_view(60.)
                    .build()?;

                objects.extend([
                    Box::new(Sphere::new(
                        Position::new(0., 240., -480.),
                        100.,
                        Box::new(Emissive::new(
                            Color::new(0.89, 0.66, 0.34),
                            15.,
                        )),
                    )) as Box<dyn Object>,
                    Box::new(Sphere::new(
                        Position::new(0., 1., -10.),
                        1.,
                        Box::new(Metal::new(
                            Color::new(0.76, 0.78, 0.78),
                            0.,
                        )),
                    )) as Box<dyn Object>,
                    Box::new(Cylinder::new(
                        Position::new(-3., 0., -8.),
                        0.75,
                        1.5,
                        Direction::new(0., 1., 0.),
                        Box::new(Dielectric::new(
                            Color::new(0.78, 0.89, 1.),
                            1.5,
                        )),
                    )),
                    Box::new(Cube::new(
                        Position::new(3., 0.75, -8.),
                        1.5,
                        Box::new(Lambertian::new(Color::new(0.02, 0.23, 0.71))),
                    )),
                    Box::new(Plane::new(
                        Position::new(0., 0., -120.),
                        Position::new(0., 1., 0.),
                        Box::new(Lambertian::new(Color::new(
                            0.8, 0.4, 0.1,
                        ))),
                    )),
                ])
            }
            4 => {
                camera = Camera::builder()
                    .origin(Position::new(8., 2., -16.))
                    .target(Position::new(0., 0., -8.))
                    .vertical_field_of_view(60.)
                    .build()?;

               
            }
            _ => {
                return Err(Error::InvalidScene(
                    "No scene corresponds to your choice",
                ))
            }
        };

        objects.sort_by_key(|object| object.depth(camera.origin()));

        Ok(Self {
            id,
            camera,
            objects,
        })
    }
}
