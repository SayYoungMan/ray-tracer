use std::any::Any;

use crate::{color::Color, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug)]
pub struct Ring {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transformation: Matrix,
}

impl Pattern for Ring {
    fn at(&self, point: crate::tuples::Point) -> Color {
        if zero_if_trivial((point.0.powi(2) + point.2.powi(2)).sqrt()).floor() % 2.0 == 0.0 {
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

    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Pattern) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Ring>() {
            self.a.equals(other.a.as_ref())
                && self.b.equals(other.b.as_ref())
                && self.transformation == other.transformation
        } else {
            false
        }
    }
}

impl Clone for Ring {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transformation: self.transformation.clone(),
        }
    }
}

impl Ring {
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
    fn ring_extends_in_both_x_and_z() {
        let ring = Ring::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(ring.at(Point::origin()), Color::white());
        assert_eq!(ring.at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(ring.at(Point::new(0.0, 0.0, 1.0)), Color::black());
        // 0.708 = just slightly more than sqrt(2)/2
        assert_eq!(ring.at(Point::new(0.708, 0.0, 0.708)), Color::black());
    }
}
