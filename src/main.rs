use {
    rt::*,
    std::io,
};

/// Promps the user to choose a scene by typing a ID corresponding
/// to a scene and Display the scene on
fn main() -> Result<()> {
    let mut input = String::new();
    println!("RAY TRACING\n");
    println!("Choose one of these scenes:");
    println!("Scene 1: 1 sphere");
    println!("Scene 2: 1 flat plane + 1 cube");
    println!("Scene 3: 1 flat plane + 1 cylinder + 1 cube + 1 sphere");
    println!("Scene 4: scene 3 with the camera in another position\n");

    loop {
        io::stdin().read_line(&mut input)?;

        match input.trim().parse() {
            Err(e) => {
                eprintln!("{e}\nPlease choose a valid integer!");
                input.clear();
                continue;
            }
            Ok(id) => match Scene::gen(id) {
                Err(e) => {
                    eprintln!("{e}Try again");
                    input.clear();
                    continue;
                }
                Ok(mut scene) => return scene.draw(),
            },
        };
    }
}
