use std::any::Any;

use crate::{color::Color, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct Ring {
    a: Color,
    b: Color,
    transformation: Matrix,
}

impl Pattern for Ring {
    fn at(&self, point: crate::tuples::Point) -> Color {
        if zero_if_trivial((point.0.powi(2) + point.2.powi(2)).sqrt()).floor() % 2.0 == 0.0 {
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
        if let Some(other) = other.as_any().downcast_ref::<Ring>() {
            self.a == other.a && self.b == other.b
        } else {
            false
        }
    }
}

impl Ring {
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
    fn ring_extends_in_both_x_and_z() {
        let ring = Ring::new(Color::white(), Color::black());

        assert_eq!(ring.at(Point::origin()), Color::white());
        assert_eq!(ring.at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(ring.at(Point::new(0.0, 0.0, 1.0)), Color::black());
        // 0.708 = just slightly more than sqrt(2)/2
        assert_eq!(ring.at(Point::new(0.708, 0.0, 0.708)), Color::black());
    }
}
