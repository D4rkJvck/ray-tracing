use rt::*;

fn main() -> Result<()> {
    let id = get_scene_id()?;
    Scene::gen(id)?.display()
}
