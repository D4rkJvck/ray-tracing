use crate::Vector;

#[allow(unused)]
pub struct FlatPlane {
    plane: Vector,
    normal: Vector,
}

impl FlatPlane {
    #[allow(unused)]
    pub fn new(plane: Vector, normal: Vector) -> Self {
        FlatPlane { plane, normal }
    }

    /*
        ? for later (potentially):
        & Intersects the plane with a ray and returns the distance along the ray to the intersection point,
        & if an intersection exists. Otherwise, returns None.
    */

    #[allow(unused)]
    // Calculates the intersection of a ray with the plane.
    pub fn intersect(&self, ray_origin: &Vector, ray_direction: &Vector) -> Option<f64> {
        let denominator = ray_direction.dot(self.normal);

        if denominator.abs() > 1e-6 {
            let ray = self.plane - *ray_origin;
            let t = ray.dot(self.normal) / denominator;
            if t >= 0.0 { Some(t.into()) }
            else { None }
        } else { None }
    }
}
