use rt::{
    Camera,
    Color,
    Position as pos,
    Scene,
    Sphere,
};

fn main() {
    let camera = Camera::new(pos::new(0.0, 0.0, 0.0),
                             2.0,
                             1.0);

    let scene = Scene::new(camera,
                           vec![Sphere::new(pos::new(0.0, 0.0, -1.0),
                                            0.5,
                                            Color::new(1.0, 0.0, 0.0))]);
    scene.display();
}
