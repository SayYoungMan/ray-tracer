use std::any::Any;

use crate::{color::Color, matrices::Matrix, tuples::Point};

use super::Pattern;

#[derive(Debug)]
pub struct Stripe {
    a: Box<dyn Pattern>,
    b: Box<dyn Pattern>,
    transformation: Matrix,
}

impl Pattern for Stripe {
    fn at(&self, point: Point) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
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
        if let Some(other) = other.as_any().downcast_ref::<Stripe>() {
            self.a.equals(other.a.as_ref())
                && self.b.equals(other.b.as_ref())
                && self.transformation == other.transformation
        } else {
            false
        }
    }
}

impl Clone for Stripe {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            transformation: self.transformation.clone(),
        }
    }
}

impl Stripe {
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
    use crate::{
        patterns::solid::Solid,
        shapes::{sphere::Sphere, Shape},
        transformation::{scaling, translation},
    };

    use super::*;

    #[test]
    fn creating_stripe_pattern() {
        let stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert!(stripe.a.equals(&Solid::new(Color::white())));
        assert!(stripe.b.equals(&Solid::new(Color::black())));
    }

    #[test]
    fn stripe_is_constant_in_y() {
        let stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(stripe.at(Point::origin()), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_is_constant_in_z() {
        let stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        assert_eq!(stripe.at(Point::origin()), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(stripe.at(Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_alternates_in_x() {
        let stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

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
        let stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );

        let c = stripe.at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::new();
        let mut stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );
        stripe.set_transformation(scaling(2.0, 2.0, 2.0));

        let c = stripe.at_object(&object, Point::new(1.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transformation(scaling(2.0, 2.0, 2.0));

        let mut stripe = Stripe::new(
            Box::new(Solid::new(Color::white())),
            Box::new(Solid::new(Color::black())),
        );
        stripe.set_transformation(translation(0.5, 0.0, 0.0));

        let c = stripe.at_object(&object, Point::new(2.5, 0.0, 0.0));

        assert_eq!(c, Color::white());
    }
}
