use crate::{
    Color,
    Position,
    Direction,
};

pub struct Light {
    position: Position,
    color: Color,
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

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    pub fn calculate_lighting(&self, point: Position, normal: Direction) -> Color {
        let light_dir = (self.position - point).unit();
        let diffuse = normal.dot(light_dir).max(0.0);
        
        self.color * (diffuse * self.intensity)
    }
}