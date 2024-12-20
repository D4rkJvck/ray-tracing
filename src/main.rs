use rt::{
    Camera,
    Position,
    Scene,
};

fn main() {
    let camera = Camera::new(Position::new(0.0, 0.0, 0.0),
                             2.0,
                             1.0);

    let scene = Scene::new(camera, vec![]);
    scene.display();
}
