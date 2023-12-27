use std::any::Any;

use crate::{color::Color, matrices::Matrix, utils::zero_if_trivial};

use super::Pattern;

#[derive(Debug, Clone)]
pub struct Solid {
    color: Color,
    transformation: Matrix,
}

impl Pattern for Solid {
    fn at(&self, _: crate::tuples::Point) -> Color {
        self.color
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
        if let Some(other) = other.as_any().downcast_ref::<Solid>() {
            self.color == other.color
        } else {
            false
        }
    }
}

impl Solid {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            transformation: Matrix::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Point;

    use super::*;

    #[test]
    fn same_color_everywhere() {
        let solid = Solid::new(Color::white());

        assert_eq!(solid.at(Point::origin()), Color::white());
        assert_eq!(solid.at(Point::new(0.25, 0.0, 0.0)), Color::white());
        assert_eq!(solid.at(Point::new(0.5, 0.0, 0.0)), Color::white());
        assert_eq!(solid.at(Point::new(0.75, 0.0, 0.0)), Color::white());
    }
}
