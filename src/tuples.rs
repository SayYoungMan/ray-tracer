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

impl std::ops::Add for SpatialTuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl std::ops::Sub for SpatialTuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
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

    #[test]
    fn adding_two_tuples() {
        let a1 = SpatialTuple(3.0, -2.0, 5.0, 1.0);
        let a2 = SpatialTuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, SpatialTuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = SpatialTuple::new_point(3.0, 2.0, 1.0);
        let p2 = SpatialTuple::new_point(5.0, 6.0, 7.0);

        // Notice that you get a vector from subtracting two points
        assert_eq!(p1 - p2, SpatialTuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = SpatialTuple::new_point(3.0, 2.0, 1.0);
        let v = SpatialTuple::new_vector(5.0, 6.0, 7.0);

        // Conceptually, this is just moving backward by the given vector
        assert_eq!(p - v, SpatialTuple::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = SpatialTuple::new_vector(3.0, 2.0, 1.0);
        let v2 = SpatialTuple::new_vector(5.0, 6.0, 7.0);

        // This represents the change in direction between the two
        assert_eq!(v1 - v2, SpatialTuple::new_vector(-2.0, -4.0, -6.0));
    }
}
