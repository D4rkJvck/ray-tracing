use {
    super::Scene,
    crate::{
        common::Error,
        Camera,
        Image,
        Object,
        Result,
    },
};

pub struct SceneBuilder {
    id:       u8,
    camera:   Option<Camera>,
    objects:  Vec<Box<dyn Object>>,
    img_size: (i32, i32),
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            id:       0,
            camera:   None,
            objects:  vec![],
            img_size: (0, 0),
        }
    }
}

impl SceneBuilder {
    pub fn id(mut self, id: u8) -> Self {
        self.id = id;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn add_objects(mut self, objects: Vec<Box<dyn Object>>) -> Self {
        self.objects.extend(objects);
        self.objects
            .sort_by_key(|object| -object.depth());
        self
    }

    pub fn image_size(mut self, width: i32, height: i32) -> Self {
        self.img_size = (width, height);
        self
    }

    fn width(&self) -> usize { self.img_size.0 as usize }

    fn height(&self) -> usize { self.img_size.1 as usize }

    pub fn build(self) -> Result<Scene> {
        let image = Image::new(self.width(), self.height())?;

        if self.camera.is_none() {
            return Err(Error::InvalidScene("Missing camera"));
        };

        Ok(Scene::new(
            self.id,
            self.camera.unwrap(),
            self.objects,
            image,
        ))
    }
}
