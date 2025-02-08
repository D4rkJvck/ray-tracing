use rt::*;

fn main() -> Result<()> {
    let mut input = String::new();

    println!("Choose one of these scenes:");
    println!("1. Scene with a sphere");
    println!("2. Scene with a flat plane and a cube");
    println!("3. Scene with one of each of all the objects");
    println!("4. Scene 3 with the camera in another position");

    loop {
        io::stdin().read_line(&mut input)?;

        match input.trim().parse() {
            Err(_) => {
                println!("Please choose a valid integer!");
                input.clear();
                continue;
            }
            Ok(id) => match Scene::gen(id) {
                Err(e) => {
                    println!("{e}Try again");
                    input.clear();
                    continue;
                }
                Ok(mut scene) => return scene.display(),
            },
        };
    }
}
