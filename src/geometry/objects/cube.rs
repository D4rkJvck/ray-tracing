use {
    super::{Impact, Object},
    crate::{material::Material, optics::Ray, Direction, Position},
};

pub struct Cube {
    center: Position,
    size: f64,
    material: Box<dyn Material>,
}

impl Cube {
    pub fn new(center: Position, size: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            size,
            material,
        }
    }

    fn get_axis_value(&self, v: &Position, axis: usize) -> f64 {
        match axis {
            0 => v.x(),
            1 => v.y(),
            2 => v.z(),
            _ => unreachable!(),
        }
    }

    fn get_slab_intersection(&self, ray: &Ray, axis: usize) -> Option<(f64, f64)> {
        let half_size = self.size / 2.0;
        let center_on_axis = self.get_axis_value(&self.center, axis);
        let min = center_on_axis - half_size;
        let max = center_on_axis + half_size;

        let origin = self.get_axis_value(&ray.origin(), axis);
        let direction = self.get_axis_value(&ray.direction(), axis);

        if direction.abs() < 1e-6 {
            // Ray is parallel to the slab
            return if origin < min || origin > max {
                None
            } else {
                Some((f64::NEG_INFINITY, f64::INFINITY))
            };
        }

        let mut t1 = (min - origin) / direction;
        let mut t2 = (max - origin) / direction;

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        Some((t1, t2))
    }
}

impl Object for Cube {
    fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    fn position(&self) -> Position {
        self.center
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let mut t_near = f64::NEG_INFINITY;
        let mut t_far = f64::INFINITY;
        let mut hit_axis = 0;
        let mut hit_is_max = false;

        // Check intersections with the three pairs of parallel planes (slabs)
        for axis in 0..3 {
            if let Some((t1, t2)) = self.get_slab_intersection(ray, axis) {
                if t1 > t_near {
                    t_near = t1;
                    hit_axis = axis;
                    hit_is_max = false;
                }
                if t2 < t_far {
                    t_far = t2;
                    if t2 == t1 {
                        hit_axis = axis;
                        hit_is_max = true;
                    }
                }

                if t_near > t_far || t_far < t_min || t_near > t_max {
                    return None;
                }
            } else {
                return None;
            }
        }

        let t = if t_near > t_min { t_near } else { t_far };
        if t < t_min || t > t_max {
            return None;
        }

        // Create normal vector based on the hit face
        let normal = match hit_axis {
            0 => Direction::new(if hit_is_max { 1.0 } else { -1.0 }, 0.0, 0.0),
            1 => Direction::new(0.0, if hit_is_max { 1.0 } else { -1.0 }, 0.0),
            2 => Direction::new(0.0, 0.0, if hit_is_max { 1.0 } else { -1.0 }),
            _ => unreachable!(),
        };

        Some(ray.generate_impact(normal, t))
    }
}
