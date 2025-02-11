use {
    super::{
        Impact,
        Object,
    },
    crate::{
        material::Material,
        optics::Ray,
        Direction,
        Position,
    },
};

pub struct Cylinder {
    center:      Position,
    radius:      f64,
    height:      f64,
    orientation: Direction,
    material:    Box<dyn Material>,
}

impl Cylinder {
    pub fn new(
        center: Position,
        radius: f64,
        height: f64,
        orientation: Direction,
        material: Box<dyn Material>,
    ) -> Self {
        Self {
            center,
            radius,
            height,
            orientation: orientation.unit(),
            material,
        }
    }

    fn is_within_radius(&self, point: Position) -> bool {
        let to_point = point - self.center;
        let height = to_point.dot(self.orientation);
        let projected = self.center + height * self.orientation;
        let radius_vector = point - projected;
        radius_vector.dot(radius_vector) <= self.radius * self.radius
    }
}

impl Object for Cylinder {
    fn material(&self) -> &dyn Material { self.material.as_ref() }

    fn position(&self) -> Position { self.center }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Impact> {
        let oc = ray.origin() - self.center;

        // Check side surface
        let dot_dir_orient = ray
            .direction()
            .dot(self.orientation);
        let dot_oc_orient = oc.dot(self.orientation);

        let a = ray
            .direction()
            .dot(ray.direction())
            - dot_dir_orient * dot_dir_orient;
        let b = 2.0 * (ray.direction().dot(oc) - dot_dir_orient * dot_oc_orient);
        let c = oc.dot(oc) - dot_oc_orient * dot_oc_orient - self.radius * self.radius;

        let mut closest_hit = None;
        let mut closest_t = t_max;

        // Check cylinder side
        if a.abs() >= 1e-6 {
            let discriminant = b * b - 4.0 * a * c;
            if discriminant >= 0.0 {
                let root = discriminant.sqrt();
                let t1 = (-b - root) / (2.0 * a);
                let t2 = (-b + root) / (2.0 * a);

                for t in [t1, t2].iter() {
                    if *t >= t_min && *t <= closest_t {
                        let hit_point = ray.cast(*t);
                        let height_vec = hit_point - self.center;
                        let height = height_vec.dot(self.orientation);

                        if height >= 0.0 && height <= self.height {
                            let projected = self.center + height * self.orientation;
                            let normal = (hit_point - projected).unit();
                            closest_hit = Some(ray.generate_impact(normal, *t));
                            closest_t = *t;
                        }
                    }
                }
            }
        }

        // Check top and bottom caps
        for h in [0.0, self.height].iter() {
            let cap_center = self.center + *h * self.orientation;
            let denom = ray
                .direction()
                .dot(self.orientation);

            if denom.abs() >= 1e-6 {
                let t = (cap_center - ray.origin()).dot(self.orientation) / denom;
                if t >= t_min && t <= closest_t {
                    let hit_point = ray.cast(t);
                    if self.is_within_radius(hit_point) {
                        let normal = if *h == 0.0 {
                            -self.orientation
                        }
                        else {
                            self.orientation
                        };
                        closest_hit = Some(ray.generate_impact(normal, t));
                        closest_t = t;
                    }
                }
            }
        }

        closest_hit
    }
}
