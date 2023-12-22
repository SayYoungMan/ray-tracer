use std::error::Error;

use crate::{matrices::Matrix, tuples::Point};

#[allow(non_snake_case, unused)]
pub fn matrix_experiments() -> Result<(), Box<dyn Error>> {
    // It stays as identity matrix (I * I.inverse() = I)
    println!("Inverse of identity: {:?}", Matrix::identity().inverse());

    let A = Matrix::from_vec(vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ]);
    println!("Original matrix: {:?}", A);
    // It will return identity matrix (A * A.inverse() = I)
    println!(
        "Multiply matrix by its inverse: {:?}",
        A.clone() * A.inverse()
    );

    let A = Matrix::from_vec(vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ]);
    // They are equal
    println!(
        "Inverse of transpose: {:?}",
        A.clone().transpose().inverse()
    );
    println!("Transpose of inverse: {:?}", A.inverse().transpose());

    let A = Matrix::from_vec(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 2.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);
    let b = Point(1.0, 2.0, 3.0, 1.0);
    // This doubles only the second element of the tuple
    println!("Second of identity matrix is 2: {:?}", A * b);

    Ok(())
}
