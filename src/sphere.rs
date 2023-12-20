use crate::tuples::{new_point, SpatialTuple};

pub struct Sphere {
    pub center: SpatialTuple,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: SpatialTuple, radius: f64) -> Self {
        Sphere {
            center,
            radius,
        }
    }

    pub fn origin_unit_sphere() -> Self {
        Sphere::new(new_point(0.0, 0.0, 0.0), 1.0)
    }
}
