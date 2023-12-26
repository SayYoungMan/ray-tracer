use std::any::Any;

use crate::{color::Color, matrices::Matrix};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct Gradient {
    a: Color,
    b: Color,
    transformation: Matrix,
}

impl Pattern for Gradient {
    fn new(color_a: Color, color_b: Color) -> Self
    where
        Self: Sized,
    {
        Self {
            a: color_a,
            b: color_b,
            transformation: Matrix::identity(),
        }
    }

    fn at(&self, point: crate::tuples::Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.0 - point.0.floor();

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
        if let Some(other) = other.as_any().downcast_ref::<Gradient>() {
            self.a == other.a && self.b == other.b
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Point;

    use super::*;

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let gradient = Gradient::new(Color::white(), Color::black());

        assert_eq!(gradient.at(Point::origin()), Color::white());
        assert_eq!(
            gradient.at(Point::new(0.25, 0.0, 0.0)),
            Color(0.75, 0.75, 0.75)
        );
        assert_eq!(gradient.at(Point::new(0.5, 0.0, 0.0)), Color(0.5, 0.5, 0.5));
        assert_eq!(
            gradient.at(Point::new(0.75, 0.0, 0.0)),
            Color(0.25, 0.25, 0.25)
        );
    }
}
