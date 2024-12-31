use crate::{
    geometry::Impact,
    Color,
    Direction,
    Position,
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

    pub fn illuminate(&self, impact: &Impact) -> Color {
        let light_dir = (self.position - impact.point).unit();

        let diffuse = impact
            .normal
            .dot(light_dir)
            .max(0.0);

        self.color * (diffuse * self.intensity)
    }

    pub fn position(&self) -> Position { self.position }

    pub fn color(&self) -> Color { self.color }

    pub fn intensity(&self) -> f64 { self.intensity }
}
