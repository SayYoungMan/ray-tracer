use crate::matrices::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let data = vec![
        vec![1.0, 0.0, 0.0, x],
        vec![0.0, 1.0, 0.0, y],
        vec![0.0, 0.0, 1.0, z],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let data = vec![
        vec![x, 0.0, 0.0, 0.0],
        vec![0.0, y, 0.0, 0.0],
        vec![0.0, 0.0, z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

pub fn rotation_x(r: f64) -> Matrix {
    let data = vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, r.cos(), -r.sin(), 0.0],
        vec![0.0, r.sin(), r.cos(), 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

pub fn rotation_y(r: f64) -> Matrix {
    let data = vec![
        vec![r.cos(), 0.0, r.sin(), 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![-r.sin(), 0.0, r.cos(), 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

pub fn rotation_z(r: f64) -> Matrix {
    let data = vec![
        vec![r.cos(), -r.sin(), 0.0, 0.0],
        vec![r.sin(), r.cos(), 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
    let data = vec![
        vec![1.0, x_y, x_z, 0.0],
        vec![y_x, 1.0, y_z, 0.0],
        vec![z_x, z_y, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ];

    Matrix::from_vec(data)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::tuples::{new_point, new_vector};

    use super::*;

    #[test]
    fn multiplying_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = new_point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_inverse_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = new_point(-3.0, 4.0, 5.0);

        // Inverse of translation matrix takes the point in reverse direction
        assert_eq!(inv * p, new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = new_vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = new_point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = new_vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = new_vector(-4.0, 6.0, 8.0);

        // This will scale the tuple in the opposite way
        assert_eq!(inv * v, new_vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_opposite_direction() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            new_point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = new_point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn sharing_transformation_moves_one_in_proportion_to_another() {
        // x in proportion to y
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(5.0, 3.0, 4.0));

        // x in proportion to z
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(6.0, 3.0, 4.0));

        // y in proportion to x
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(2.0, 5.0, 4.0));

        // y in proportion to z
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(2.0, 7.0, 4.0));

        // z in proportion to x
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(2.0, 3.0, 6.0));

        // z in proportion to y
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(2.0, 3.0, 7.0));
    }
}
