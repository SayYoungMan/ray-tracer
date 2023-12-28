use std::any::Any;

use crate::{color::Color, constants::EPSILON, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug)]
pub struct Blended {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transformation: Matrix,
}

impl Pattern for Blended {
    fn at(&self, point: crate::tuples::Point) -> Color {
        let local_pattern_point_a = self.a.transformation().inverse() * point;
        let local_pattern_point_b = self.b.transformation().inverse() * point;

        self.a.at(local_pattern_point_a) * self.b.at(local_pattern_point_b)
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
        if let Some(other) = other.as_any().downcast_ref::<Blended>() {
            self.a.equals(other.a.as_ref())
                && self.b.equals(other.b.as_ref())
                && self.transformation == other.transformation
        } else {
            false
        }
    }

    fn clone_box(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }
}

impl Clone for Blended {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transformation: self.transformation.clone(),
        }
    }
}

impl Blended {
    pub fn new(a: Box<dyn Pattern>, b: Box<dyn Pattern>) -> Self {
        Self {
            a,
            b,
            transformation: Matrix::identity(),
        }
    }
}
