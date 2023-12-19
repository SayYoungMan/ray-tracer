use crate::constants::EPSILON;

#[derive(Debug)]
pub struct Color(f64, f64, f64);

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPSILON
            && (self.1 - other.1).abs() < EPSILON
            && (self.2 - other.2).abs() < EPSILON
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);

        // This is used to blend two colors together
        assert_eq!(c1 * c2, Color(0.9, 0.2, 0.04));
    }
}
