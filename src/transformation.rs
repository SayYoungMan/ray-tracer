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

#[cfg(test)]
mod tests {
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
}
