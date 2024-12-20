use rt::{
    Camera,
    Color,
    // FlatPlane,
    Position as pos,
    Scene,
    Sphere,
};

fn main() {
    let camera = Camera::new(pos::new(0.0, 0.0, 0.0),
                             2.0,
                             1.0);

    let scene_for_sphere = Scene::new(camera,
                           vec![Sphere::new(pos::new(0.0, 0.0, -1.0),
                                            0.5,
                                            Color::new(1.0, 0.0, 0.0))]);
    scene_for_sphere.display();

    // let scene_for_plane = Scene::new(camera,
    //                        vec![FlatPlane::new(pos::new(0.0, -0.5, 0.0),
    //                        pos::new(0.0, 1.0, 0.0),
    //                        Color::new(0.0, 1.0, 0.0))]);
    // scene_for_plane.display();
}
