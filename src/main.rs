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

};

fn main() {
    let mut scene = Scene::new(
       Camera::new(Position::default()),
    //    Camera::new(Position::new(0.0, 1.0, 0.0)),

        vec![
            Light::new(
                Position::new(2.0, 2.0, -1.0),
                Color::new(1.0, 1.0, 1.0),
                1.0,
            ),
            // Light::new(
            //     Position::new(-2.0, 1.0, -1.0),
            //     Color::new(0.5, 0.5, 1.0),
            //     0.6,
            // ),
        ],
        vec![
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -2.0),
                0.5,
                Color::new(0.1, 0.0, 0.0),
            )),
            // Box::new(Cylinder::new(
            //     Position::new(2.0, 0.0, -2.0),  
            //     0.5,                              
            //     1.6,                              
            //     Direction::new(0.0, 1.0, 0.0),   
            //     Color::new(0.2, 0.2, 0.2),        
            // )),

            Box::new(Cylinder::new(
                Position::new(2.0, -0.5, -3.0),  
                0.5,                              
                1.6,                              
                Direction::new(0.0, 1.0, 0.0),   
                Color::new(0.2, 0.2, 0.2),        
            )),
            
           
            Box::new(FlatPlane::new(
                Position::new(0.0, -0.5, -1.0), // Position the plane slightly below the camera
                Position::new(0.0, 1.0, 0.0),   // Use a unit vector pointing upwards as the normal
                Color::new(0.1, 0.1, 0.0),
            )),
        
            
        ],
    );

    scene.display();
}
