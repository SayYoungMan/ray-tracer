use crate::{
    matrices::Matrix,
    tuples::{Point, Vector},
};

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

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let upn = up.normalize();

    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix::from_vec(vec![
        vec![left.0, left.1, left.2, 0.0],
        vec![true_up.0, true_up.1, true_up.2, 0.0],
        vec![-forward.0, -forward.1, -forward.2, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    orientation * Transformation::Translation(-from.0, -from.1, -from.2).matrix()
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

    mod view_transform {
        use super::*;

        #[test]
        fn default_orientation() {
            let from = Point::origin();
            let to = Point::new(0.0, 0.0, -1.0);
            let up = Vector::new(0.0, 1.0, 0.0);

            let t = view_transform(from, to, up);

            assert_eq!(t, Matrix::identity());
        }

        #[test]
        fn looking_in_positive_z_direction() {
            let from = Point::origin();
            let to = Point::new(0.0, 0.0, 1.0);
            let up = Vector::new(0.0, 1.0, 0.0);

            let t = view_transform(from, to, up);

            assert_eq!(t, Transformation::Scaling(-1.0, 1.0, -1.0).matrix());
        }

        #[test]
        fn view_transform_moves_world() {
            let from = Point::new(0.0, 0.0, 8.0);
            let to = Point::origin();
            let up = Vector::new(0.0, 1.0, 0.0);

            let t = view_transform(from, to, up);

            assert_eq!(t, Transformation::Translation(0.0, 0.0, -8.0).matrix());
        }

        #[test]
        fn arbitrary_view_transform() {
            let from = Point::new(1.0, 3.0, 2.0);
            let to = Point::new(4.0, -2.0, 8.0);
            let up = Vector::new(1.0, 1.0, 0.0);

            let t = view_transform(from, to, up);

            assert_eq!(
                t,
                Matrix::from_vec(vec![
                    vec![-0.50709, 0.50709, 0.67612, -2.36643],
                    vec![0.76772, 0.60609, 0.12122, -2.82843],
                    vec![-0.35857, 0.59761, -0.71714, 0.0],
                    vec![0.0, 0.0, 0.0, 1.0]
                ])
            );
        }
    }
}
