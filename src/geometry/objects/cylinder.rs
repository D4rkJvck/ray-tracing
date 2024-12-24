use {
    super::{Impact, Object},
    crate::{
        optics::Ray,
        Color,
        Direction,
        Position,
    },
};

pub struct Cylinder {
    center:      Position,
    radius:      f64,
    height:      f64,
    orientation: Direction,
    color:       Color,
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

    fn hit(&self, ray: &Ray, _t_min: f64, _t_max: f64, impact: &mut Impact) -> bool {
        let oc = ray.origin() - self.center;

        let a = ray
            .direction()
            .dot(ray.direction())
            - ray
                .direction()
                .dot(self.orientation)
                .powf(2.0);

        let b = 2.0
            * (ray.direction().dot(oc)
                - ray
                    .direction()
                    .dot(self.orientation)
                    * oc.dot(self.orientation));

        let c = oc.dot(oc)
            - oc.dot(self.orientation)
                .powf(2.0)
            - self.radius.powf(2.0);

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let check_height = |t: f64| -> bool {
            if t < 0.0 {
                return false;
            }

            let point = ray.cast(t);
            let height_vec = point - self.center;
            let height = height_vec.dot(self.orientation);

            height >= 0.0 && height <= self.height
        };

        let t_base = |height: f64| -> f64 {
            let base_center = self.center + height * self.orientation;
            let denom = ray
                .direction()
                .dot(self.orientation);

            if denom.abs() < 1e-6 {
                return -1.0;
            }

            let t =
                (base_center - ray.origin()).dot(self.orientation) / denom;
            if t < 0.0 {
                return -1.0;
            }

            let point = ray.cast(t);
            if (point - base_center).length() <= self.radius {
                t
            }
            else {
                -1.0
            }
        };

        let t_bottom = t_base(0.0);
        let t_top = t_base(self.height);

        let mut t = -1.0;
        for candidate_t in [t1, t2, t_bottom, t_top].iter() {
            if *candidate_t > 0.0 && (t < 0.0 || *candidate_t < t) {
                if check_height(*candidate_t)
                    || *candidate_t == t_bottom
                    || *candidate_t == t_top
                {
                    t = *candidate_t;
                }
            }
        }
        true
    }
}
