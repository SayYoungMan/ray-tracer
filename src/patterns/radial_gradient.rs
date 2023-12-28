use std::any::Any;

use crate::{color::Color, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct RadialGradient {
    a: Color,
    b: Color,
    transformation: Matrix,
}

impl Pattern for RadialGradient {
    fn at(&self, point: crate::tuples::Point) -> Color {
        let distance = self.b - self.a;
        let radial_distance_from_center = (point.0.powi(2) + point.2.powi(2)).sqrt();
        let fraction =
            radial_distance_from_center - zero_if_trivial(radial_distance_from_center).floor();

        self.a + distance * fraction
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
        if let Some(other) = other.as_any().downcast_ref::<RadialGradient>() {
            self.a == other.a && self.b == other.b
        } else {
            false
        }
    }
}

impl RadialGradient {
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
