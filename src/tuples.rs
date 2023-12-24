use crate::{constants::EPSILON, transformation};

pub trait Tuple {
    fn translate(self, x: f64, y: f64, z: f64) -> Self;

    fn scale(self, x: f64, y: f64, z: f64) -> Self;

    fn rotate_x(self, r: f64) -> Self;

    fn rotate_y(self, r: f64) -> Self;

    fn rotate_z(self, r: f64) -> Self;

    fn shear(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub f64, pub f64, pub f64, pub f64);

impl Tuple for Vector {
    fn translate(self, x: f64, y: f64, z: f64) -> Self {
        transformation::translation(x, y, z) * self
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Self {
        transformation::scaling(x, y, z) * self
    }

    fn rotate_x(self, r: f64) -> Self {
        transformation::rotation_x(r) * self
    }

    fn rotate_y(self, r: f64) -> Self {
        transformation::rotation_y(r) * self
    }

    fn rotate_z(self, r: f64) -> Self {
        transformation::rotation_z(r) * self
    }

    fn shear(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        transformation::shearing(x_y, x_z, y_x, y_z, z_x, z_y) * self
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(x, y, z, 0.0)
    }

    pub fn from_vec(vec: Vec<f64>) -> Self {
        if vec.len() != 4 {
            panic!("Invalid length of vector received: {:?}", vec);
        }

        Vector(vec[0], vec[1], vec[2], vec[3])
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();

        self / mag
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(&normal)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
            && (self.3 - other.3).abs() < EPSILON
    }
}

impl std::ops::Add for Vector {
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

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl std::ops::Sub for Vector {
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

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl std::ops::Mul<f64> for Vector {
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

impl std::ops::Div<f64> for Vector {
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

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64, pub f64);

impl Tuple for Point {
    fn translate(self, x: f64, y: f64, z: f64) -> Self {
        transformation::translation(x, y, z) * self
    }

    fn scale(self, x: f64, y: f64, z: f64) -> Self {
        transformation::scaling(x, y, z) * self
    }

    fn rotate_x(self, r: f64) -> Self {
        transformation::rotation_x(r) * self
    }

    fn rotate_y(self, r: f64) -> Self {
        transformation::rotation_y(r) * self
    }

    fn rotate_z(self, r: f64) -> Self {
        transformation::rotation_z(r) * self
    }

    fn shear(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        transformation::shearing(x_y, x_z, y_x, y_z, z_x, z_y) * self
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(x, y, z, 1.0)
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }

    pub fn from_vec(vec: Vec<f64>) -> Self {
        if vec.len() != 4 {
            panic!("Invalid length of vector received: {:?}", vec);
        }

        Point(vec[0], vec[1], vec[2], vec[3])
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
            && (self.3 - other.3).abs() < EPSILON
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn point_creates_tuple_with_w1() {
        let p = Point::new(4.0, -4.0, 3.0);

        assert_eq!(p, Point(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w0() {
        let v = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(v, Vector(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Point(3.0, -2.0, 5.0, 1.0);
        let a2 = Vector(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Point(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        // Notice that you get a vector from subtracting two points
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        // Conceptually, this is just moving backward by the given vector
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        // This represents the change in direction between the two
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_tuple() {
        let a = Vector(1.0, -2.0, 3.0, -4.0);
        let b = Point(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Vector(-1.0, 2.0, -3.0, 4.0));
        assert_eq!(-b, Point(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_vector_by_scalar() {
        let a = Vector(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Vector(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_vector_by_fraction() {
        let a = Vector(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, Vector(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let a = Vector(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, Vector(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_magnitude_of_vectors() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());

        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vectors() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vector::new(1.0, 0.0, 0.0));

        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);

        assert_eq!(
            p.rotate_x(PI / 2.0)
                .scale(5.0, 5.0, 5.0)
                .translate(10.0, 5.0, 7.0),
            Point::new(15.0, 0.0, 7.0)
        );
    }

    #[test]
    fn reflecting_vector_at_45deg() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
