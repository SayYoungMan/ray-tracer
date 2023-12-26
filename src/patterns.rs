use crate::{color::Color, matrices::Matrix, shapes::Shape, tuples::Point};
use std::{any::Any, fmt::Debug};

pub mod gradient;
pub mod ring;
pub mod stripe;

pub trait Pattern: Debug {
    fn new(color_a: Color, color_b: Color) -> Self
    where
        Self: Sized;

    fn at(&self, point: Point) -> Color;

    fn at_object(&self, object: &dyn Shape, world_point: Point) -> Color {
        let object_point = object.transformation().inverse() * world_point;
        let pattern_point = self.transformation().inverse() * object_point;

        self.at(pattern_point)
    }

    fn transformation(&self) -> Matrix;

    fn set_transformation(&mut self, m: Matrix);

    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn Pattern) -> bool;

    fn clone_box(&self) -> Box<dyn Pattern>;
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(&**other)
    }
}

#[derive(Debug)]
struct TestPattern {
    transformation: Matrix,
}

impl Pattern for TestPattern {
    fn new(_: Color, _: Color) -> Self
    where
        Self: Sized,
    {
        Self {
            transformation: Matrix::identity(),
        }
    }

    fn at(&self, _: Point) -> Color {
        todo!()
    }

    fn transformation(&self) -> Matrix {
        todo!()
    }

    fn set_transformation(&mut self, m: Matrix) {
        self.transformation = m;
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn equals(&self, _: &dyn Pattern) -> bool {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Pattern> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::transformation::translation;

    use super::*;

    #[test]
    fn default_pattern_transformation() {
        let pattern = TestPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.transformation, Matrix::identity());
    }

    #[test]
    fn assigning_transformation() {
        let mut pattern = TestPattern::new(Color::white(), Color::black());
        pattern.set_transformation(translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.transformation, translation(1.0, 2.0, 3.0));
    }
}
