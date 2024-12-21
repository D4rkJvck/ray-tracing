use rt::{
    Camera,
    Color,
    // FlatPlane,
    Position,
    Scene,
    Sphere,
    // Cylinder,
    // Direction,
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
            Box::new(Sphere::new(
                Position::new(0.0, 0.0, -1.0),  // Position originale de la sphère
                0.5,
                Color::new(1.0, 0.0, 0.0),
            )),

              // Box::new(FlatPlane::new(
            //     Position::new(0.0, -0.5, 0.0),
            //     Position::new(0.0, 0.1, 0.0),
            //     Color::new(0.0, 1.0, 0.0),
            // )),

            // Box::new(Cylinder::new(
            //     Position::new(0.8, -0.5, -1.0), 
            //     0.2,                             
            //     1.0,                            
            //     Direction::new(0.0, 1.0, 0.0), 
            //     Color::new(0.8, 0.0, 0.8),      
            // )),
        ],
    );

    scene_for_plane.display();
}