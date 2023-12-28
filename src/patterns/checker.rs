use std::any::Any;

use crate::{color::Color, constants::EPSILON, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug)]
pub struct Checker {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transformation: Matrix,
}

impl Pattern for Checker {
    fn at(&self, point: crate::tuples::Point) -> Color {
        let (x, y, z) = (
            zero_if_trivial(point.0),
            zero_if_trivial(point.1),
            zero_if_trivial(point.2),
        );

        if (x.floor() as i32 + y.floor() as i32 + z.floor() as i32) % 2 == 0 {
            let local_pattern_point = self.a.transformation().inverse() * point;
            return self.a.at(local_pattern_point);
        }

        let local_pattern_point = self.b.transformation().inverse() * point;
        self.b.at(local_pattern_point)
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transformation = m;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Pattern) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Checker>() {
            self.a.equals(other.a.as_ref()) && self.b.equals(other.b.as_ref())
        } else {
            false
        }
    }

    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }
}

impl Clone for Checker {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transformation: self.transformation.clone(),
        }
    }
}

impl Checker {
    pub fn new(a: Box<dyn Pattern>, b: Box<dyn Pattern>) -> Self {
        Self {
            a,
            b,
            transformation: Matrix::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{patterns::solid::Solid, tuples::Point};

    use super::*;

    #[test]
    fn checker_should_repeat_in_x() {
        let checker = Checker::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(checker.at(Point::new(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checker_should_repeat_in_y() {
        let checker = Checker::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checker_should_repeat_in_z() {
        let checker = Checker::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.0, 1.01)), Color::black());
    }
}
