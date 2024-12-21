use {
    super::Object,
    crate::{
        optics::Ray,
        Color,
        Position,
        Direction,
    },
};

pub struct Cylinder {
    center: Position,     
    radius: f32,         
    height: f32,         
    direction: Direction, 
    color: Color,        
}

impl Cylinder {
    pub fn new(center: Position, radius: f32, height: f32, direction: Direction, color: Color) -> Self {
        Self {
            center,
            radius,
            height,
            direction: direction.normal(),
            color,
        }
    }
}

impl Object for Cylinder {
    fn color(&self) -> Color { 
        self.color 
    }

    fn hit(&self, ray: &Ray) -> f32 {
        let oc = ray.origin() - self.center;
        
        let a = ray.direction().dot(ray.direction()) - 
                ray.direction().dot(self.direction).powf(2.0);
                
        let b = 2.0 * (ray.direction().dot(oc) - 
                ray.direction().dot(self.direction) * oc.dot(self.direction));
                
        let c = oc.dot(oc) - oc.dot(self.direction).powf(2.0) - self.radius.powf(2.0);

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let check_height = |t: f32| -> bool {
            if t < 0.0 {
                return false;
            }
            let point = ray.cast(t);
            let height_vec = point - self.center;
            let height = height_vec.dot(self.direction);
            height >= 0.0 && height <= self.height
        };

        let t_base = |height: f32| -> f32 {
            let base_center = self.center + height * self.direction;
            let denom = ray.direction().dot(self.direction);
            if denom.abs() < 1e-6 {
                return -1.0;
            }
            let t = (base_center - ray.origin()).dot(self.direction) / denom;
            if t < 0.0 {
                return -1.0;
            }
            let point = ray.cast(t);
            if (point - base_center).magnitude() <= self.radius {
                t
            } else {
                -1.0
            }
        };

        let t_bottom = t_base(0.0);
        let t_top = t_base(self.height);

        let mut t = -1.0;
        for candidate_t in [t1, t2, t_bottom, t_top].iter() {
            if *candidate_t > 0.0 && (t < 0.0 || *candidate_t < t) {
                if check_height(*candidate_t) || *candidate_t == t_bottom || *candidate_t == t_top {
                    t = *candidate_t;
                }
            }
        }
        t
    }
}