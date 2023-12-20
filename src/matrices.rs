use crate::{constants::EPSILON, tuples::SpatialTuple};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

#[allow(dead_code)]
impl Matrix {
    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();

        for row in &data {
            if row.len() != cols {
                panic!("The number of columns should equal in all rows.");
            }
        }

        Matrix { rows, cols, data }
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn transpose(self) -> Self {
        let mut data = Vec::new();
        for j in 0..self.cols {
            let mut row = Vec::new();
            for i in 0..self.rows {
                row.push(self.at(i, j));
            }
            data.push(row);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    fn determinant(&self) -> f64 {
        if self.cols != 2 || self.rows != 2 {
            panic!(
                "Determinant method only supports 2x2 matrix, recieved: {:#?}",
                self
            );
        }

        let a = self.at(0, 0);
        let b = self.at(0, 1);
        let c = self.at(1, 0);
        let d = self.at(1, 1);

        a * d - b * c
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut data = Vec::new();
        for i in 0..self.rows {
            if i == row {
                continue;
            }

            let mut tmp_row = Vec::new();
            for j in 0..self.cols {
                if j == col {
                    continue;
                }

                tmp_row.push(self.at(i, j));
            }
            data.push(tmp_row);
        }

        Matrix::from_vec(data)
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        if self.cols != 3 || self.rows != 3 {
            panic!(
                "Minor method only supports 3x3 matrix, recieved: {:#?}",
                self
            );
        }

        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        }
        return -self.minor(row, col);
    }

    pub fn identity() -> Self {
        Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.cols != other.cols || self.rows != other.rows {
            return false;
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.at(i, j) - other.at(i, j)) > EPSILON {
                    return false;
                }
            }
        }

        return true;
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Matrix) -> Self::Output {
        if self.rows != other.cols {
            panic!("The matrix dimensions should match to perform multiplication: self.rows: {}, other.cols: {}", self.rows, other.cols);
        }

        let mut data = Vec::new();
        for i in 0..self.rows {
            let mut row_vector = Vec::new();
            for j in 0..other.cols {
                let mut sum = 0.0;
                for n in 0..self.rows {
                    sum += self.at(i, n) * other.at(n, j);
                }
                row_vector.push(sum);
            }
            data.push(row_vector);
        }

        Matrix::from_vec(data)
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: &Matrix) -> Self::Output {
        if self.rows != other.cols {
            panic!("The matrix dimensions should match to perform multiplication: self.rows: {}, other.cols: {}", self.rows, other.cols);
        }

        let mut data = Vec::new();
        for i in 0..self.rows {
            let mut row_vector = Vec::new();
            for j in 0..other.cols {
                let mut sum = 0.0;
                for n in 0..self.rows {
                    sum += self.at(i, n) * other.at(n, j);
                }
                row_vector.push(sum);
            }
            data.push(row_vector);
        }

        Matrix::from_vec(data)
    }
}

impl std::ops::Mul<SpatialTuple> for Matrix {
    type Output = SpatialTuple;

    fn mul(self, other: SpatialTuple) -> Self::Output {
        if self.cols != 4 {
            panic!(
                "The number of columns of matrix must be 4 but got {}",
                self.cols
            );
        }

        let mut data = Vec::new();
        for i in 0..self.rows {
            let sum = self.at(i, 0) * other.0
                + self.at(i, 1) * other.1
                + self.at(i, 2) * other.2
                + self.at(i, 3) * other.3;
            data.push(sum);
        }

        SpatialTuple::from_vec(data)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];

        let M = Matrix::from_vec(data);

        assert_eq!(M.at(0, 0), 1.0);
        assert_eq!(M.at(0, 3), 4.0);
        assert_eq!(M.at(1, 0), 5.5);
        assert_eq!(M.at(1, 2), 7.5);
        assert_eq!(M.at(2, 2), 11.0);
        assert_eq!(M.at(3, 0), 13.5);
        assert_eq!(M.at(3, 2), 15.5);
    }

    #[test]
    fn other_size_matrices_representable() {
        // 2x2 Matrix
        let data = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let M = Matrix::from_vec(data);

        assert_eq!(M.at(0, 0), -3.0);
        assert_eq!(M.at(0, 1), 5.0);
        assert_eq!(M.at(1, 0), 1.0);
        assert_eq!(M.at(1, 1), -2.0);

        // 3x3 Matrix
        let data = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];
        let M = Matrix::from_vec(data);

        assert_eq!(M.at(0, 0), -3.0);
        assert_eq!(M.at(1, 1), -2.0);
        assert_eq!(M.at(2, 2), 1.0);
    }

    #[test]
    fn matrix_equality_with_indentical_matrices() {
        let A = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(A, B);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let A = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::from_vec(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(A, B);
    }

    #[test]
    fn multiplying_two_matrices() {
        let A = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::from_vec(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::from_vec(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(A * B, expected);
    }

    #[test]
    fn multiplying_matrix_by_tuple() {
        let A = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let b = SpatialTuple(1.0, 2.0, 3.0, 1.0);

        let expected = SpatialTuple(18.0, 24.0, 33.0, 1.0);

        assert_eq!(A * b, expected);
    }

    #[test]
    fn multiplying_by_identity_matrix() {
        let A = Matrix::from_vec(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);
        let a = SpatialTuple(1.0, 2.0, 3.0, 4.0);
        let identity_matrix = Matrix::identity();

        assert_eq!(A.clone() * &identity_matrix, A);
        assert_eq!(identity_matrix * a, a);
    }

    #[test]
    fn transposing_matrix() {
        let A = Matrix::from_vec(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix::from_vec(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(A.transpose(), expected);
    }

    #[test]
    fn transposing_identity_matrix() {
        let A = Matrix::identity().transpose();

        assert_eq!(A, Matrix::identity());
    }

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let A = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);

        assert_eq!(A.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let A = Matrix::from_vec(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let expected = Matrix::from_vec(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(A.submatrix(0, 2), expected);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let A = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(A.submatrix(2, 1), expected);
    }

    #[test]
    fn calculating_minor_of_3x3() {
        let A = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let B = A.submatrix(1, 0);

        assert_eq!(B.determinant(), 25.0);
        assert_eq!(A.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_cofactor_of_3x3() {
        let A = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert_eq!(A.minor(0, 0), -12.0);
        assert_eq!(A.cofactor(0, 0), -12.0);
        assert_eq!(A.minor(1, 0), 25.0);
        assert_eq!(A.cofactor(1, 0), -25.0);
    }
}
