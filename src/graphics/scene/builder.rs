use {
    super::Scene,
    crate::{
        common::Error,
        Camera,
        Light,
        Object,
        Result,
    },
};

pub struct SceneBuilder {
    id:      u8,
    camera:  Option<Camera>,
    lights:  Vec<Light>,
    objects: Vec<Box<dyn Object>>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            id:      0,
            camera:  None,
            lights:  vec![],
            objects: vec![],
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

    pub fn add_lights(mut self, lights: Vec<Light>) -> Self {
        self.lights.extend(lights);
        self
    }

    pub fn add_objects(mut self, objects: Vec<Box<dyn Object>>) -> Self {
        self.objects.extend(objects);
        self.objects
            .sort_by_key(|object| -object.depth());
        self
    }

    pub fn build(self) -> Result<Scene> {
        if self.camera.is_none() {
            return Err(Error::InvalidScene("Missing camera"));
        };

        Ok(Scene::new(
            self.id,
            self.camera.unwrap(),
            self.lights,
            self.objects,
        ))
    }
}
