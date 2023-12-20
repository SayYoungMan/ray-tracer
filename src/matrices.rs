#[derive(Debug)]
pub struct Matrix(Vec<Vec<f64>>);

#[allow(dead_code)]
impl Matrix {
    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        Matrix(data)
    }

    pub fn at(&self, row: usize, col: usize) -> f64 {
        self[row][col]
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
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
}
