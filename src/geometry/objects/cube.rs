use {
    super::{
        Impact,
        Object,
    },
    crate::{
        optics::Ray,
        Color,
        Position,
        Vector,
    },
};

// use crate::geometry::{Impact, Object, Position, Direction};
// use crate::Color;
// use crate::optics::Ray;

pub struct Cube {
    pub position: Position,
    pub size:     f64,
    pub color:    Color,
}

impl Cube {
    pub fn new(position: Position, size: f64, color: Color) -> Self {
        Self {
            position,
            size,
            color: color.unit(),
        }
    }
}

impl Object for Cube {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.position }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, impact: &mut Impact) -> bool {
        let half_size = Vector::new(
            self.size / 2.0,
            self.size / 2.0,
            self.size / 2.0,
        );
        let min = self.position - half_size;
        let max = self.position + half_size;

        let mut t_min_current = t_min;
        let mut t_max_current = t_max;

        for i in 0..3 {
            let inv_d = 1.0
                / match i {
                    0 => ray.direction().x(),
                    1 => ray.direction().y(),
                    2 => ray.direction().z(),
                    _ => unreachable!(),
                };

            let mut t0 = (match i {
                0 => min.x() - ray.origin().x(),
                1 => min.y() - ray.origin().y(),
                2 => min.z() - ray.origin().z(),
                _ => unreachable!(),
            }) * inv_d;

            let mut t1 = (match i {
                0 => max.x() - ray.origin().x(),
                1 => max.y() - ray.origin().y(),
                2 => max.z() - ray.origin().z(),
                _ => unreachable!(),
            }) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min_current = t_min_current.max(t0);
            t_max_current = t_max_current.min(t1);

            if t_max_current <= t_min_current {
                return false;
            }
        }

        impact.t = t_min_current;
        impact.point = ray.cast(impact.t);
        impact.set_face_normal(
            ray.direction(),
            (impact.point - self.position).unit(),
        );

        true
    }
}
