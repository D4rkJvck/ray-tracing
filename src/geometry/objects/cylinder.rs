use {
    super::{
        Impact,
        Object,
    },
    crate::{
        optics::Ray,
        Color,
        Direction,
        Position,
    },
};


pub struct Cylinder {
    center: Position,
    radius: f64,
    height: f64,
    orientation: Direction,
    color: Color,
}

impl Cylinder {
    pub fn new(
        center: Position,
        radius: f64,
        height: f64,
        orientation: Direction,
        color: Color,
    ) -> Self {
        Self {
            center,
            radius,
            height,
            orientation: orientation.unit(),
            color: color.unit(),
        }
    }
}

impl Object for Cylinder {
    fn color(&self) -> Color { self.color }

    fn position(&self) -> Position { self.center }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, impact: &mut Impact) -> bool {
        let oc = ray.origin() - self.center;
        
        // Calculate coefficients for quadratic equation
        let a = ray.direction().dot(ray.direction()) 
            - ray.direction().dot(self.orientation).powf(2.0);
            
        let b = 2.0 * (ray.direction().dot(oc) 
            - ray.direction().dot(self.orientation) * oc.dot(self.orientation));
            
        let c = oc.dot(oc) 
            - oc.dot(self.orientation).powf(2.0) 
            - self.radius.powf(2.0);

        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return false;
        }

        let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_min || t > t_max {
                return false;
            }
        }

        // Calculate hit point
        let hit_point = ray.cast(t);
        
        // Check if hit point is within cylinder height
        let height_vec = hit_point - self.center;
        let height = height_vec.dot(self.orientation);
        
        if height < 0.0 || height > self.height {
            return false;
        }

        impact.t = t;
        impact.point = hit_point;

        // Calculate surface normal
        let projected = self.center + height * self.orientation;
        let normal = (hit_point - projected).unit();
        impact.set_face_normal(ray.direction(), normal);

        true
    }
}