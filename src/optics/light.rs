use {
    super::{
        Impact,
        Ray,
    },
    crate::{
        Color,
        Object,
        Position,
    },
    std::f64::INFINITY,
};

pub struct Light {
    position:  Position,
    color:     Color,
    intensity: f64,
}

impl Light {
    pub fn new(position: Position, color: Color, intensity: f64) -> Self {
        Self {
            position,
            color: color.unit(),
            intensity,
        }
    }

    pub fn illuminate(&self, impact: &Impact, objects: &Vec<Box<dyn Object>>) -> Color {
        let light_dir = (self.position - impact.point).unit();
        let shadow_ray = Ray::new(impact.point, light_dir);

        // Check if the shadow ray hits any object
        for object in objects {
            if object.hit(
                &shadow_ray,
                0.001,
                INFINITY,
                &mut Impact::new(),
            ) {
                // If there's an intersection, the point is in shadow
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let diffuse = impact
            .surface_normal
            .dot(light_dir)
            .max(0.0);

        self.color * (diffuse * self.intensity)
    }

    pub fn position(&self) -> Position { self.position }

    pub fn color(&self) -> Color { self.color }

    pub fn intensity(&self) -> f64 { self.intensity }
}
