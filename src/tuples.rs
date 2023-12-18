#[derive(Debug, PartialEq)]
struct Point(f64, f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 1.0)
    }
}

#[derive(Debug, PartialEq)]
struct Vector(f64, f64, f64, f64);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 0.0)
    }
}

#[cfg(test)]
mod tests {
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
}
