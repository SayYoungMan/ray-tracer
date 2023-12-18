const EPSILON: f64 = 1e-5;

pub fn new_point(x: f64, y: f64, z: f64) -> SpatialTuple {
    SpatialTuple(x, y, z, 1.0)
}

pub fn new_vector(x: f64, y: f64, z: f64) -> SpatialTuple {
    SpatialTuple(x, y, z, 0.0)
}

#[derive(Debug)]
pub struct SpatialTuple(f64, f64, f64, f64);

impl SpatialTuple {
    fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
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

impl std::ops::Neg for SpatialTuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl std::ops::Mul<f64> for SpatialTuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
            self.3 * scalar,
        )
    }
}

impl std::ops::Div<f64> for SpatialTuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self(
            self.0 / scalar,
            self.1 / scalar,
            self.2 / scalar,
            self.3 / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creates_tuple_with_w1() {
        let p = new_point(4.0, -4.0, 3.0);

        assert_eq!(p, SpatialTuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w0() {
        let v = new_vector(4.0, -4.0, 3.0);

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
        let p1 = new_point(3.0, 2.0, 1.0);
        let p2 = new_point(5.0, 6.0, 7.0);

        // Notice that you get a vector from subtracting two points
        assert_eq!(p1 - p2, new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = new_point(3.0, 2.0, 1.0);
        let v = new_vector(5.0, 6.0, 7.0);

        // Conceptually, this is just moving backward by the given vector
        assert_eq!(p - v, new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = new_vector(3.0, 2.0, 1.0);
        let v2 = new_vector(5.0, 6.0, 7.0);

        // This represents the change in direction between the two
        assert_eq!(v1 - v2, new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_tuple() {
        let a = SpatialTuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, SpatialTuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = SpatialTuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, SpatialTuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = SpatialTuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, SpatialTuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = SpatialTuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, SpatialTuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_magnitude_of_vectors() {
        let v = new_vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = new_vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = new_vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = new_vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());

        let v = new_vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }
}
