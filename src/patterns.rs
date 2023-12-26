use crate::{color::Color, matrices::Matrix, shapes::Shape, tuples::Point};
use std::{any::Any, fmt::Debug};

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

#[derive(Debug, Clone)]
pub struct Stripe {
    a: Color,
    b: Color,
    transformation: Matrix,
}

impl Pattern for Stripe {
    fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            a: color_a,
            b: color_b,
            transformation: Matrix::identity(),
        }
    }

    fn at(&self, point: Point) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
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
        if let Some(other) = other.as_any().downcast_ref::<Stripe>() {
            self.a == other.a && self.b == other.b
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::sphere::Sphere,
        transformation::{scaling, translation},
    };

    use super::*;

    #[test]
    fn creating_stripe_pattern() {
        let stripe = Stripe::new(Color::white(), Color::black());

        assert_eq!(stripe.a, Color::white());
        assert_eq!(stripe.b, Color::black());
    }

    #[test]
    fn stripe_is_constant_in_y() {
        let stripe = Stripe::new(Color::white(), Color::black());

        assert_eq!(stripe.at(Point::origin()), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_is_constant_in_z() {
        let stripe = Stripe::new(Color::white(), Color::black());

        assert_eq!(stripe.at(Point::origin()), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_alternates_in_x() {
        let stripe = Stripe::new(Color::white(), Color::black());

        assert_eq!(stripe.at(Point::origin()), Color::white());
        assert_eq!(stripe.at(Point::new(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(stripe.at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(stripe.at(Point::new(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(stripe.at(Point::new(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(stripe.at(Point::new(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = Sphere::new();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let stripe = Stripe::new(Color::white(), Color::black());

        let c = stripe.at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::new();
        let mut stripe = Stripe::new(Color::white(), Color::black());
        stripe.set_transformation(scaling(2.0, 2.0, 2.0));

        let c = stripe.at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transformation(scaling(2.0, 2.0, 2.0));
        let mut stripe = Stripe::new(Color::white(), Color::black());
        stripe.set_transformation(translation(0.5, 0.0, 0.0));

        let c = stripe.at_object(&object, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }
}
