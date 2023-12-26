use crate::{color::Color, tuples::Point};

pub trait Pattern {
    fn new(color_a: Color, color_b: Color) -> Self;

    fn at(&self, point: Point) -> Color;
}

pub struct Stripe {
    a: Color,
    b: Color,
}

impl Pattern for Stripe {
    fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            a: color_a,
            b: color_b,
        }
    }

    fn at(&self, point: Point) -> Color {
        if point.0.floor() % 2.0 == 0.0 {
            return self.a;
        }
        self.b
    }
}

#[cfg(test)]
mod tests {
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
}
