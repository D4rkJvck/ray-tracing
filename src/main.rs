use rt::{
    Camera,
    Color,
    Light,
    Position,
    Scene,
    Sphere,
    Cylinder,
    FlatPlane,
    Direction,
    Cube,
};

fn main() {
    let mut scene1 = Scene::new(
        Camera::new(Position::default()),
        vec![
            Light::new(
                Position::new(2.0, 2.0, -1.0),
                Color::new(1.0, 1.0, 1.0),
                1.0,
            ),
        ],
        vec![
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -2.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),
            Box::new(FlatPlane::new(
                Position::new(0.0, -0.5, -1.0),
                Position::new(0.0, 1.0, 0.0),
                Color::new(0.1, 0.1, 0.0),
            )),
        ],
    );
    scene1.display("scene1_output.ppm");

    let mut scene2 = Scene::new(
        Camera::new(Position::default()),
        vec![
            Light::new(
                Position::new(2.0, 2.0, -1.0),
                Color::new(1.0, 1.0, 1.0),
                0.5, 
            ),
        ],
        vec![
            Box::new(FlatPlane::new(
                Position::new(0.0, -0.5, -1.0), 
                Position::new(0.0, 1.0, 0.0),   
                Color::new(0.5, 0.5, 0.5),      
            )),
            Box::new(Cube::new(
                Position::new(-1.5, 0.0, -2.5),
                1.0,                           
                Color::new(0.0, 0.2, 0.5),      
            )),
        ],
    );
    scene2.display("scene2_output.ppm");

    let mut scene3 = Scene::new(
        Camera::new(Position::default()),
        vec![
            Light::new(
                Position::new(2.0, 2.0, -1.0),
                Color::new(1.0, 1.0, 1.0),
                1.0, 
            ),
        ],
        vec![
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -2.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),

            Box::new(Cylinder::new(
                Position::new(2.0, -0.5, -3.0),
                0.5,
                1.6,
                Direction::new(0.0, 1.0, 0.0),
                Color::new(0.2, 0.2, 0.2),
            )),

            Box::new(FlatPlane::new(
                Position::new(0.0, -0.5, -1.0),
                Position::new(0.0, 1.0, 0.0),
                Color::new(0.1, 0.1, 0.0),
            )),

            Box::new(Cube::new(
                Position::new(-1.5, 0.0, -2.5),
                1.0,
                Color::new(0.0, 0.2, 0.5),
            )),
        ],
    );
    scene3.display("scene3_output.ppm");
    
   
}
