const EPSILON: f64 = 1e-5;

#[derive(Debug)]
struct SpatialTuple(f64, f64, f64, f64);

impl SpatialTuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 0.0)
    }
}

impl PartialEq for SpatialTuple {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
            && (self.3 - other.3).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_tuple_with_w1() {
        let p = SpatialTuple::new_point(4.0, -4.0, 3.0);

        assert_eq!(p, SpatialTuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w0() {
        let v = SpatialTuple::new_vector(4.0, -4.0, 3.0);

        assert_eq!(v, SpatialTuple(4.0, -4.0, 3.0, 0.0));
    }
}
