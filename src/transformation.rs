use crate::matrices::Matrix;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Transformation {
    Translation(f64, f64, f64),
    Scaling(f64, f64, f64),
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
    Shearing(f64, f64, f64, f64, f64, f64),
}

impl Transformation {
    pub fn matrix(&self) -> Matrix {
        match self {
            Transformation::Translation(x, y, z) => {
                let data = vec![
                    vec![1.0, 0.0, 0.0, *x],
                    vec![0.0, 1.0, 0.0, *y],
                    vec![0.0, 0.0, 1.0, *z],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
            Transformation::Scaling(x, y, z) => {
                let data = vec![
                    vec![*x, 0.0, 0.0, 0.0],
                    vec![0.0, *y, 0.0, 0.0],
                    vec![0.0, 0.0, *z, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
            Transformation::RotationX(r) => {
                let data = vec![
                    vec![1.0, 0.0, 0.0, 0.0],
                    vec![0.0, r.cos(), -r.sin(), 0.0],
                    vec![0.0, r.sin(), r.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
            Transformation::RotationY(r) => {
                let data = vec![
                    vec![r.cos(), 0.0, r.sin(), 0.0],
                    vec![0.0, 1.0, 0.0, 0.0],
                    vec![-r.sin(), 0.0, r.cos(), 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
            Transformation::RotationZ(r) => {
                let data = vec![
                    vec![r.cos(), -r.sin(), 0.0, 0.0],
                    vec![r.sin(), r.cos(), 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
            Transformation::Shearing(x_y, x_z, y_x, y_z, z_x, z_y) => {
                let data = vec![
                    vec![1.0, *x_y, *x_z, 0.0],
                    vec![*y_x, 1.0, *y_z, 0.0],
                    vec![*z_x, *z_y, 1.0, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0],
                ];

                Matrix::from_vec(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::tuples::{Point, Vector};

    use super::*;

    #[test]
    fn multiplying_translation_matrix() {
        let transform = Transformation::Translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(transform.matrix() * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_inverse_translation_matrix() {
        let transform = Transformation::Translation(5.0, -3.0, 2.0);
        let inv = transform.matrix().inverse();
        let p = Point::new(-3.0, 4.0, 5.0);

        // Inverse of translation matrix takes the point in reverse direction
        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_not_affect_vectors() {
        let transform = Transformation::Translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform.matrix() * v, v);
    }

    #[test]
    fn scaling_matrix_to_point() {
        let transform = Transformation::Scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);

        assert_eq!(transform.matrix() * p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_to_vector() {
        let transform = Transformation::Scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(transform.matrix() * v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_inverse_of_scaling_matrix() {
        let transform = Transformation::Scaling(2.0, 3.0, 4.0);
        let inv = transform.matrix().inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);

        // This will scale the tuple in the opposite way
        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative() {
        let transform = Transformation::Scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::RotationX(PI / 4.0);
        let full_quarter = Transformation::RotationX(PI / 2.0);

        assert_eq!(
            half_quarter.matrix() * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter.matrix() * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_opposite_direction() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::RotationX(PI / 4.0);
        let inv = half_quarter.matrix().inverse();

        assert_eq!(
            inv * p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transformation::RotationY(PI / 4.0);
        let full_quarter = Transformation::RotationY(PI / 2.0);

        assert_eq!(
            half_quarter.matrix() * p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter.matrix() * p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transformation::RotationZ(PI / 4.0);
        let full_quarter = Transformation::RotationZ(PI / 2.0);

        assert_eq!(
            half_quarter.matrix() * p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter.matrix() * p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn sharing_transformation_moves_one_in_proportion_to_another() {
        // x in proportion to y
        let transform = Transformation::Shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(5.0, 3.0, 4.0));

        // x in proportion to z
        let transform = Transformation::Shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(6.0, 3.0, 4.0));

        // y in proportion to x
        let transform = Transformation::Shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(2.0, 5.0, 4.0));

        // y in proportion to z
        let transform = Transformation::Shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(2.0, 7.0, 4.0));

        // z in proportion to x
        let transform = Transformation::Shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(2.0, 3.0, 6.0));

        // z in proportion to y
        let transform = Transformation::Shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_eq!(transform.matrix() * p, Point::new(2.0, 3.0, 7.0));
    }
}
