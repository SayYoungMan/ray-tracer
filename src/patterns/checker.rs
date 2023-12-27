use std::any::Any;

use crate::{color::Color, constants::EPSILON, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct Checker {
    a: Color,
    b: Color,
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
            return self.a;
        }
        self.b
    }

    fn transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transformation = m;
    }

    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Pattern) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Checker>() {
            self.a == other.a && self.b == other.b
        } else {
            false
        }
    }
}

impl Checker {
    pub fn new(color_a: Color, color_b: Color) -> Self
    where
        Self: Sized,
    {
        Self {
            a: color_a,
            b: color_b,
            transformation: Matrix::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Point;

    use super::*;

    #[test]
    fn checker_should_repeat_in_x() {
        let checker = Checker::new(Color::white(), Color::black());

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(checker.at(Point::new(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checker_should_repeat_in_y() {
        let checker = Checker::new(Color::white(), Color::black());

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checker_should_repeat_in_z() {
        let checker = Checker::new(Color::white(), Color::black());

        assert_eq!(checker.at(Point::origin()), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(checker.at(Point::new(0.0, 0.0, 1.01)), Color::black());
    }
}
